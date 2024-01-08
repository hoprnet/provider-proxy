mod providers;

use crate::providers::get_provider;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::str::Utf8Error;

pub use console_error_panic_hook::set_once as set_panic_hook;

use worker::*;

#[derive(Serialize, Deserialize, Debug)]
struct EthRequest {
    method: String,
    jsonrpc: String,
    params: Vec<String>,
    id: i32,
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
    let hyper_response = sender.send_request(request).await.map_err(map_hyper_error)?;
    let buf = hyper::body::to_bytes(hyper_response).await.map_err(map_hyper_error)?;
    let text = std::str::from_utf8(&buf).map_err(map_utf8_error)?;
    let mut response = Response::ok(text)?;
    response.headers_mut().append("Content-Type", "application/json")?;
    let ret_code = response.status_code();
    console_log!(
        "Forwarded request to {}, processed in ? ms, called method {}, returned http code {}",
        provider_name,
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

    // TODO: add elapsed time check

    let url = Url::parse(endpoint.url.as_str())?;
    let socket = Socket::builder()
        .secure_transport(SecureTransport::On)
        .connect(url.domain().unwrap(), url.port_or_known_default().unwrap())?;

    let (sender, connection) = hyper::client::conn::handshake(socket).await.map_err(map_hyper_error)?;

    let eth_request = req.clone_mut()?.json().await;

    if eth_request.is_err() {
        let e = eth_request.unwrap_err();
        console_error!("Bad request body: {:?}", e);
        return Response::error("Bad request", 400);
    }

    let data: EthRequest = eth_request.unwrap();
    let mut request = hyper::Request::builder()
        .uri(endpoint.url.clone())
        .method("POST")
        .header("Host", url.domain().unwrap());

    if let Some(auth_token) = endpoint.auth_token.clone() {
        request = request.header("Authorization", format!("Bearer {}", auth_token));
    }

    let final_request = request
        .body(hyper::Body::from(serde_json::to_string(&data).unwrap()))
        .map_err(map_hyper_http_error)?;

    tokio::select!(
        res = connection => {
            console_error!("Connection exited: {:?}", res);
            Err(worker::Error::RustError("Connection exited".to_string()))
        },
        result = make_request(sender, final_request, provider_name, data.method) => result
    )
}
