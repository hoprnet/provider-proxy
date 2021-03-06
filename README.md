# provider-proxy

This is a small Cloudflare worker library which implements a configuration-based
web-proxy used to map keys to full URLs. In particular its used to map
ETH API provider names with ETH API provider endpoint URLs.

The configuration of providers is done in `src/providers.ts`.

## Setup

In order to test and develop this library locally, one must setup `wrangler`
first. To do that one must first instantiate the configuration file
`wrangler.toml.example` to `wranger.toml` and replace the values `PLACEHOLDER`
with proper data.

Then it may be configured further by running:

```
yarn
yarn wrangler login
```

## Test

All providers are tested by running `yarn && yarn test`.

## Deploy

Just upload the current state to Cloudflare. It will become live immediately.

```
yarn wrangler publish
```

## View Production Logs

You may watch logs from the deployed and running workers:

```
yarn wrangler tail --format=pretty
```
