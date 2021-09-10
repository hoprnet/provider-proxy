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

  const requestJson = await request.clone().json();
  let isMulticall = false;
  let method = requestJson.method;
  if (Array.isArray(requestJson)) {
    isMulticall = true;
    method = requestJson[0].method;
  }
  const [, providerUrl] = provider;
  const start = Date.now();

  return fetch(providerUrl, request).then(async function (response) {
    const elapsed = Date.now() - start;
    const responseClone = response.clone();
    const responseCloneJson = await responseClone.json();
    let result = "ok";
    if (isMulticall) {
      const callCount = requestJson.length;
      const errorResponse = responseCloneJson.find(
        (r) => r.error !== undefined
      );
      if (errorResponse !== undefined) {
        result = `at least 1 error ${errorResponse.error.message}`;
      }
      console.log(
        `Forwarded multi-call request to ${path}, processed in ${elapsed} ms, called method ${method}, returned ${result}, included ${callCount} calls`
      );
    } else {
      if (responseCloneJson.error !== undefined) {
        result = `error ${responseCloneJson.error.message}`;
      }
      console.log(
        `Forwarded request to ${path}, processed in ${elapsed} ms, called method ${method}, returned ${result}`
      );
    }

    return response;
  });
}
