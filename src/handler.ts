import { providers } from "./providers";

export async function handleRequest(request: Request): Promise<Response> {
  // get URL path without the prepended slash
  const path = new URL(request.url).pathname.slice(1);

  // fail early if method is not supported
  if (request.method != "POST") {
    console.log(`Unsupported request: ${request.url}`)
    return new Response("not found", { status: 405 });
  }

  // handle only known providers
  const provider = Object.entries(providers).find(([k]) => k === path);
  if (provider !== undefined) {
    const [, providerUrl] = provider;
    const start = Date.now();
    return await fetch(providerUrl, request).then(function(response) {
      const elapsed = Date.now() - start;
      console.log(`Forwarded request to ${providerUrl} took ${elapsed} ms`);
      return response;
    });
  }

  // if the provider is not known, return not found
  console.log(`Unknown request: ${request.url}`)
  return new Response("not found", { status: 404 });
}
