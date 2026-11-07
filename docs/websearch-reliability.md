# Web search reliability

Automatic search tries configured SearXNG, then DuckDuckGo HTML, then Bing HTML.
Set `CODETETHER_SEARXNG_URL` to a reachable SearXNG base URL with JSON output
enabled (`search.formats: [html, json]`). Use
`CODETETHER_WEBSEARCH_BACKEND=auto` to retain fallback; explicit `searxng`,
`duckduckgo`/`ddg`, or `bing` selections intentionally disable fallback.

HTTP 200 and successful parsing are insufficient evidence of useful results.
A lexical sanity check filters results using query terms in titles, snippets,
and URLs. If none survive, `WEBSEARCH_UNRELATED_RESULTS` triggers fallback.
Every backend attempt remains in the evidence receipt; rejected results are not
published as sources. Exhausted backends return failure, not a fabricated answer.

This heuristic is not semantic ranking or geographic verification. It can reject
paraphrases and accept wrong-location results sharing enough terms. Legitimate
empty results retain successful-empty semantics. Public HTML engines can
challenge or throttle requests. SearXNG also depends on its upstream engines:
configuration alone is not proof of availability.

From the Rust workspace, run `cargo test -p codetether-websearch`. To contact real
engines without exercising the running Chat service:

```sh
CODETETHER_WEBSEARCH_BACKEND=auto cargo run -q -p codetether-websearch \
  --example live_search -- 'Rockwall TX best restaurants dinner'
```

Inspect `success`, backend HTTP statuses/outcomes, and actual source URLs across
multiple queries and repeated requests before claiming availability. Backend
activation is a separate rebuild/restart step.

Evidence: `artifacts/chat-websearch-reliability/`. `final-live-wylie.txt` records