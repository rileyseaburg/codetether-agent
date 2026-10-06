# Verifier model requests

Use a server version containing this API. The public base URL is
`https://server.codetether.run`; local development uses `http://127.0.0.1:4096`.
Provision `CODETETHER_AUTH_TOKEN` securely; never paste a token into source or a URL.
The examples keep the authorization value out of curl's process arguments.

```bash
base=https://server.codetether.run
curl -fsS -H @/dev/fd/3 "$base/api/config/verifier-model" \
  3<<<"Authorization: Bearer $CODETETHER_AUTH_TOKEN"
curl -fsS -X PUT -H @/dev/fd/3 -H 'Content-Type: application/json' \
  --data '{"model":"openai-codex/gpt-5.6-luna"}' "$base/api/config/verifier-model" \
  3<<<"Authorization: Bearer $CODETETHER_AUTH_TOKEN"
curl -fsS -X DELETE -H @/dev/fd/3 "$base/api/config/verifier-model" \
  3<<<"Authorization: Bearer $CODETETHER_AUTH_TOKEN"
```

Illustrative response immediately after PUT, before any verification has run:

```json
{
  "identity_source": "harness_configuration",
  "scope": "process",
  "persisted": false,
  "runtime_model": "openai-codex/gpt-5.6-luna",
  "environment_model": "openai-codex/gpt-5.6-luna",
  "default_model": null,
  "worker_model_context": null,
  "selected_model": "openai-codex/gpt-5.6-luna",
  "source": "runtime",
  "latest_verification": null
}
```

After a run, `latest_verification.identity_source` is
`harness_provider_resolution`. Its `requested_model` may differ from
`resolved_model` due to aliases, and `resolved_provider` names the actual adapter.
States: `resolving`, `running`, `pass`, `fail`, `unavailable`. Unavailable is a
runtime/protocol failure, not evidence of a model rejection. A `pass` is a verifier
decision, not proof the goal transition committed; a concurrent goal edit can reject it.
After DELETE the systemd-set model can still be selected with `source:environment`.

[API contract and persistence](verifier-model.md)