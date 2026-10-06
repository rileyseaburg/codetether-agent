# Verifier model API

`/api/config/verifier-model` changes the verifier in the **server process**.
It does not change the worker model, a separate TUI process, or systemd files.
Bearer authentication is mandatory. GET requires `agent:read`; PUT and DELETE
require `agent:write` (admin/operator/editor by default; viewers cannot mutate).

| Method | Request | Result |
| --- | --- | --- |
| GET | Optional `?worker_model=provider/model` | Current selection and latest harness execution metadata. |
| PUT | JSON `{"model":"openai-codex/gpt-5.6-luna"}` | Set an in-memory override for subsequent verifications. |
| DELETE | No body | Clear the runtime override; restore environment/config/worker fallback. |

PUT requires a nonempty `provider/model` identifier, no internal whitespace or
control characters, at most 512 UTF-8 bytes after trimming. Missing/null model
or unknown JSON fields return 422; invalid identifiers return 400. Mutation is
not performed on error. This is syntax validation, **not a provider inference
probe or a guarantee of model access**. Discover identifiers with `GET /models`.
401 means missing/invalid authentication, 403 insufficient policy permission,
and 500 means configuration could not be loaded. Configuration reads do not run
a model. The same settings response is returned after successful mutations.

## Selection and persistence

Precedence: process override (API or same-process TUI) →
`CODETETHER_GOAL_VERIFIER_MODEL` → configured default if different from the
worker → worker model. With no candidates, `selected_model` is null.
`worker_model_context` is a caller-supplied preview context, **not observed
worker identity**; omit it for the server-default view. GET does not persist it.
`persisted:false` applies to the API override. Restarting drops it; a systemd
environment setting survives. Concurrent changes are last-write-wins.
An in-flight verification keeps the provider/model it already resolved.

## Model identity belongs to the harness

`selected_model` is configuration, not proof that a model ran. The separate
`latest_verification` object reports the most recently **started** attempt:
requested identifier, resolved provider/model, timestamps, ID, and state.
The harness captures routing before calling the provider; model self-claims in
answer text are never used. Missing resolution stays null, including startup
failures. Later settings changes cannot rename earlier verdict-log entries.
Metadata is process-local, bounded to the latest attempt, and reset on restart.
It is not a listing of all concurrent runs or proof of a provider's internal
upstream routing. `resolved_model` is the exact model passed to its adapter.

See [examples](verifier-model-examples.md) and the [OpenAPI definition](verifier-model.openapi.yaml).
