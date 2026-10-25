# Gemini Web operational limits

The native image transport uses HTTP/1.1: its live HTTP/2 bootstrap returned a
protocol error while HTTP/1.1 authenticated with the same session. Generation
POSTs are never automatically replayed. If renewal receives HTTP 429, the client
may try authenticated bootstrap with an existing, unexpired cached session;
it does not bypass authentication or retry renewal in a loop. Revoked sessions
still require signing in and exporting fresh cookies.

Some live refinement attempts returned malformed RPC batches
(`image_generation_protocol_error: invalid_batch_json`). A later static/local
native Rust test against live Google HTTP succeeded, but the intermittent
failure's cause is unresolved. The parser fails closed and reports only a fixed
error code and JSON category/position, never the response body or signed URLs.
Existing saved versions remain intact.

Static/local native Rust checks against live Google HTTP (not deployment evidence):
- Generation and green-to-purple refinement: repository artifact
  `artifacts/gemini-rust-live-20261024T060845Z/summary.json` records the same
  conversation and a fresh response; the two JPEGs preserve the composition.
- Native renewal, private persistence, cache reuse, and authenticated bootstrap:
  `artifacts/gemini-rust-renewal-20261024T060146Z/renewal.json`.
