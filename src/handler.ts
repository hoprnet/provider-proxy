import { providers } from "./providers";

export async function handleRequest(request: Request): Promise<Response> {
  // get URL path without the prepended slash
  const path = new URL(request.url).pathname.slice(1);

  // fail early if method is not supported
  if (request.method != "POST") {
    console.log(`Unsupported request: ${request.url}`);
    return new Response("not found", { status: 405 });
  }

  // handle only known providers
  const provider = Object.entries(providers).find(([k]) => k === path);
  if (provider === undefined) {
    // if the provider is not known, return not found
    console.log(`Unknown request: ${request.url}`);
    return new Response("not found", { status: 404 });
  }
  const [, providerUrls] = provider;
  let providerUrl;
  // check if we use a single provider or round-robin over an array
  if (typeof providerUrls === "string") {
    providerUrl = providerUrls;
  } else if (Array.isArray(providerUrl)) {
    providerUrl = providerUrls[Math.floor(Math.random() * providerUrls.length)];
  } else {
    // if the provider type is not known, return not found
    console.log(`Unknown request: ${request.url}`);
    return new Response("not found", { status: 404 });
  }

  const requestJson = await request.clone().json();
  let isMulticall = false;
  let method = requestJson.method;
  if (Array.isArray(requestJson)) {
    isMulticall = true;
    method = requestJson[0].method;
  }
  const start = Date.now();

  return fetch(providerUrl, request).then(async function (response) {
    const elapsed = Date.now() - start;
    const result = `returned http code ${response.status}`;
    let responseSizeInfo = "";
    const responseSize = Number(response.headers.get("content-length"));
    if (responseSize > 0) {
      responseSizeInfo = `, response size ${responseSize} Bytes`;
    }
    if (isMulticall) {
      const callCount = requestJson.length;
      console.log(
        `Forwarded multi-call request to ${path}, processed in ${elapsed} ms, called method ${method}, ${result}, included ${callCount} calls${responseSizeInfo}`
      );
    } else {
      console.log(
        `Forwarded request to ${path}, processed in ${elapsed} ms, called method ${method}, ${result}${responseSizeInfo}`
      );
    }

    return response;
  });
}
