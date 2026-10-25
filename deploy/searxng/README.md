# SearXNG for CodeTether `websearch`

SearXNG is a backend of the built-in `websearch` tool
(`crates/codetether-websearch`), so every agent, swarm worker, router, and
`search` call can use it with no new tool to register.

## Deploy

```bash
kubectl apply -f deploy/searxng/searxng.yaml      # creates namespace first
kubectl -n searxng create secret generic searxng-secret \
  --from-literal=secret-key="$(openssl rand -hex 32)"
kubectl -n searxng rollout status deploy/searxng
# allow the worker namespace through the NetworkPolicy
kubectl label namespace <worker-namespace> codetether.run/searxng-client=true
```

## Point CodeTether at it

Set this on workers (Deployment `env`) or locally:

```bash
export CODETETHER_SEARXNG_URL=http://searxng.searxng.svc.cluster.local:8080
# optional: auto (default) | searxng | bing | duckduckgo
export CODETETHER_WEBSEARCH_BACKEND=auto
```

With `auto`, `websearch` tries SearXNG → Bing → DuckDuckGo and stops at the
first success. Use `searxng` to stop it falling back to scraping. Each
attempt appears in the `Search evidence:` JSON line. The `backend` field
there shows which backend answered.

Local test: `kubectl -n searxng port-forward svc/searxng 8080` and
`CODETETHER_SEARXNG_URL=http://localhost:8080`.

## Failure codes

| Code | Meaning |
|---|---|
| `WEBSEARCH_CONFIG_INVALID` | `searxng` selected but `CODETETHER_SEARXNG_URL` unset |
| `WEBSEARCH_PROVIDER_FORBIDDEN` | 403 — usually `json` missing from `search.formats` |
| `WEBSEARCH_UPSTREAM_UNAVAILABLE` | zero results and every SearXNG engine unresponsive |
| `WEBSEARCH_MALFORMED_RESPONSE` | body was not SearXNG JSON |
