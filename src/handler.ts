import { providers } from "./providers";

export async function handleRequest(request: Request): Promise<Response> {
  // get URL path without the prepended slash
  const path = new URL(request.url).pathname.slice(1);

  // fail early if method is not supported
  if (request.method != "POST") {
    return new Response("not found", { status: 405 });
  }

  // handle only known providers
  const provider = Object.entries(providers).find(([k]) => k === path);
  if (provider !== undefined) {
    const [, providerUrl] = provider;
    return fetch(providerUrl, request);
  }

  // if the provider is not known, return not found
  return new Response("not found", { status: 404 });
}
