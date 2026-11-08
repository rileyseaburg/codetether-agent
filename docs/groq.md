# Groq inference

Groq uses the shared OpenAI-compatible provider with the default API root
`https://api.groq.com/openai/v1`. No separate SDK or custom endpoint is required.

## Credentials

Configure the `groq` provider record through the existing Vault setup. The default
record path is `secret/codetether/providers/groq`, subject to the configured mount
and provider namespace. Set its `api_key` field using a secure Vault client or UI.
An optional `base_url` overrides the default for an explicitly trusted endpoint.
Never put a real key in source, chat, logs, or command-line arguments.

Local development also supports `GROQ_API_KEY`. Vault-loaded providers take
precedence over environment registration. `CODETETHER_DISABLE_ENV_FALLBACK=1`
disables environment credential loading, including Groq. Explicit application
configuration under `[providers.groq]` supports `api_key` and optional `base_url`;
keep any credential-bearing config private and outside version control.

## Models and inference

```sh
codetether models --provider groq --json
codetether run --model groq/openai/gpt-oss-20b "Explain Rust ownership briefly."
codetether run --model groq/openai/gpt-oss-120b "Explain Rust ownership briefly."
```

Use IDs returned by your account's live model catalog. The first `groq/` selects
the provider; model namespaces such as `openai/` are preserved in API requests.
Discovery uses authenticated `/models`, reads `context_window` and
`max_completion_tokens`, and excludes inactive and known speech/guard models.
Compound entries are not advertised as supporting client function tools.

Both streaming and non-streaming requests clamp an explicit output-token budget
to the selected model's advertised positive output limit. This applies to all
Groq models, not just Qwen; the context window is not used as the output limit.
Smaller budgets remain unchanged, and an omitted budget uses server defaults.
Successful discovery is cached per provider instance and shared with inference.
Failed or empty discovery is retried on subsequent requests. If the model or its
output limit is unavailable, the requested budget is left unchanged, so the
server may still reject it. As a temporary workaround for the reported 16,384
limit, launch with `CODETETHER_SESSION_MAX_TOKENS=16384 codetether tui`.

## Supported inference surface

- Chat completions, streamed text/reasoning, and function tool calls.
- Standard usage and Groq's `x_groq.usage` streaming metadata.
- `max_completion_tokens`, temperature, top-p, and up to four stop sequences.
- At most 128 function tools; invalid request limits fail before network I/O.
- Unsupported penalties, logprobs, metadata, and storage options are not emitted.

Reasoning effort and service tier use Groq's model/server defaults. This provider
does not add APIs for Responses beta, speech, files, batches, or fine tuning.
Model access, billing, rate limits, and model-specific capabilities remain Groq's.

Local fixture tests do not establish live credentials or model availability.
An installed version containing this source change and a configured key are
required for live use; source edits alone do not upgrade the running executable.