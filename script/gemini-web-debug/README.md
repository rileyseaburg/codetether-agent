# Gemini Web RPC diagnostic client

This client reuses the authenticated curl HTTP/2 transport to call Gemini's
private `BardFrontendService/StreamGenerate` endpoint. It is not an official
Google SDK, a browser session, or the CodeTether Rust provider.

## Run

Requirements: Node, curl with HTTP/2 support, and the existing Vault CLI access
used by `vault.mjs`. Cookies are read from
`secret/codetether/providers/gemini-web`; never put them in command arguments.

From the repository root:

```sh
node script/gemini-web-debug/rpc-retry.mjs
```

The runner requests Thinking and Pro and checks for exactly
`RPC_CLIENT_OK 1591`. It retries answer errors, missing answers, or mismatches
once. Transport, HTTP, and framing failures stop that model's attempts.
Exit 0 requires both models to return the expected answer; otherwise exit 2.

## Responsibilities

- `rpc-client.mjs`: session acquisition, model selection, authenticated requests.
- `rpc-framing.mjs`: incremental anti-XSSI and length-prefixed batch decoder.
- `rpc-lengths.mjs`: credential-free framing-length evidence.
- `rpc-frames.mjs`: `wrb.fr` answer extraction and observed error-answer detection.
- `rpc-result.mjs`: separate transport, HTTP, framing, RPC, and answer outcomes.
- `rpc-retry.mjs`: bounded retries and restricted-permission evidence artifacts.

Opaque `e` trailers are not interpreted as error codes. HTTP 200 alone is not
success. These responses use framed JSON, not SSE; requesting SSE does not
change the protocol. Configured mode IDs do not attest backend model versions.

## Tests and evidence

```sh
node --test script/gemini-web-debug/rpc-framing.test.mjs script/gemini-web-debug/rpc-result.test.mjs script/gemini-web-debug/curl-response.test.mjs
```

Tests are mocked local. Live HTTP diagnostics write redacted JSON under
`artifacts/gemini-rpc-client-*/`, with directory/file permissions 0700/0600.

## Recorded retry

Static/local client diagnostic against live Google HTTP:
`artifacts/gemini-rpc-client-20261013T225327143Z/summary.json`.

- Pro (`9d8ca3786ebdfbea`) returned exactly `RPC_CLIENT_OK 1591` in 15.77 seconds.
- Thinking (`5bf011840784117a`) returned generic error answers on both attempts.
- All three transfers returned HTTP 200 over HTTP/2 with valid RPC framing.
  The overall runner exited 2 because Thinking did not satisfy the answer check.

Observed length prefixes include the two newlines surrounding each JSON batch;
they are not just the JSON byte length. The decoder and mocked-local fixtures
account for those bytes. No raw response or request credentials were saved.
This retry does not establish Rust-provider integration or backend identity.