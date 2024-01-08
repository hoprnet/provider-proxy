mod providers;

use crate::providers::get_provider;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::str::Utf8Error;

pub use console_error_panic_hook::set_once as set_panic_hook;

use worker::*;

#[derive(Serialize, Deserialize, Debug)]
enum RequestBody {
    EthRequest(EthRequest),
    EthRequestBatch(Vec<EthRequest>),
}

#[derive(Serialize, Deserialize, Debug)]
struct EthRequest {
    method: String,
    jsonrpc: String,
    params: Vec<String>,
    id: i32,
}

impl RequestBody {
    fn get_method(&self) -> String {
        match self {
            RequestBody::EthRequest(r) => r.method.clone(),
            RequestBody::EthRequestBatch(_) => "batch".to_string(),
        }
    }
}

fn map_utf8_error(error: Utf8Error) -> worker::Error {
    worker::Error::RustError(format!("Utf8Error: {:?}", error))
}

fn map_hyper_error(error: hyper::Error) -> worker::Error {
    worker::Error::RustError(format!("hyper::Error: {:?}", error))
}

fn map_hyper_http_error(error: hyper::http::Error) -> worker::Error {
    worker::Error::RustError(format!("hyper::http::Error: {:?}", error))
}

async fn make_request(
    mut sender: hyper::client::conn::SendRequest<hyper::Body>,
    request: hyper::Request<hyper::Body>,
    provider_name: &str,
    method: String,
) -> Result<Response> {
    let start = Utc::now();
    let hyper_response = sender.send_request(request).await.map_err(map_hyper_error)?;
    let end = Utc::now();
    let elapsed = end.timestamp_millis() - start.timestamp_millis();
    let buf = hyper::body::to_bytes(hyper_response).await.map_err(map_hyper_error)?;
    let text = std::str::from_utf8(&buf).map_err(map_utf8_error)?;
    let mut response = Response::ok(text)?;
    response.headers_mut().append("Content-Type", "application/json")?;
    let ret_code = response.status_code();
    console_log!(
        "Forwarded request to {}, processed in {} ms, called method {}, returned http code {}",
        provider_name,
        elapsed,
        method,
        ret_code
    );
    Ok(response)
}

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    set_panic_hook();

    // fail early if method is not supported
    if !matches!(req.method(), Method::Post) {
        console_log!("Unsupported request: {}", req.path());
        return Response::error("Method not allowed", 405);
    }
    let path = req.path();

    // handle only known providers
    let provider_name = path.splitn(2, '/').last().unwrap();
    let provider = get_provider(provider_name);
    if provider.is_none() {
        console_log!("Unsupported request: {}", path);
        return Response::error("not found", 404);
    }

    // Use request content-length to choose an endpoint semi-randomly.
    // Thus, requests with the same content length will go to the same endpoint.
    let endpoints = provider.unwrap().endpoints;
    let content_length = req
        .headers()
        .get("content-length")?
        .map_or(0, |l| usize::from_str(l.as_str()).unwrap_or(0));
    let endpoint_pos: usize = content_length % endpoints.len();
    let endpoint = endpoints.get(endpoint_pos).unwrap_or(endpoints.first().unwrap());

    let url = Url::parse(endpoint.url.as_str())?;
    let socket = Socket::builder()
        .secure_transport(SecureTransport::On)
        .connect(url.domain().unwrap(), url.port_or_known_default().unwrap())?;

    let (sender, connection) = hyper::client::conn::handshake(socket).await.map_err(map_hyper_error)?;

    let (method, data) = match req.clone_mut()?.json::<RequestBody>().await {
        Ok(v) => (v.get_method(), serde_json::to_string(&v)),
        Err(e) => {
            console_error!("Bad request body: {:?}", e);
            return Response::error("Bad request", 400);
        }
    };

    let mut request = hyper::Request::builder()
        .uri(endpoint.url.clone())
        .method("POST")
        .header("Host", url.domain().unwrap());

    if let Some(auth_token) = endpoint.auth_token.clone() {
        request = request.header("Authorization", format!("Bearer {}", auth_token));
    }

    let final_request = request
        .body(hyper::Body::from(data.unwrap()))
        .map_err(map_hyper_http_error)?;

    tokio::select!(
        res = connection => {
            console_error!("Connection exited: {:?}", res);
            Err(worker::Error::RustError("Connection exited".to_string()))
        },
        result = make_request(sender, final_request, provider_name, method) => result
    )
}
