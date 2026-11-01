# LM Studio runtime budgets and admission

These settings require a binary containing the provider changes. Editing an
environment file does not patch an older running binary. Restart the chat server
only after installing a matching, checksum-verified CI artifact.

| Environment variable | Default | Allowed range |
|---|---:|---:|
| `CODETETHER_LMSTUDIO_MAX_OUTPUT_TOKENS` | 4096 | 1–32768 |
| `CODETETHER_LMSTUDIO_QUEUE_CAPACITY` | 16 waiting | 0–128 |
| `CODETETHER_LMSTUDIO_QUEUE_TIMEOUT_SECS` | 120 | 1–600 |
| `CODETETHER_LMSTUDIO_REQUEST_TIMEOUT_SECS` | 600 | 1–600 |

The effective answer cap is the minimum of the caller's requested maximum,
the configured cap, one quarter of the **loaded** context window, and estimated
remaining space after reserving 20% of the window. A 32768-token loaded window
therefore advertises 4096 output tokens by default; configuring 8192 allows 8192
only when the request actually leaves sufficient space. Invalid settings fail
closed. Original system/user/tool input is not silently dropped by this guard.

One generation runs at a time per normalized HTTP origin, shared by provider
instances and `lmstudio`/`lm-studio` aliases **within one CodeTether process**.
Other providers are unaffected. Separate processes and clients that call LM Studio
directly do not share this gate. LM Studio controls cancellation of GPU work after
an HTTP disconnect; the client cannot promise that GPU work stops instantaneously.

Admission is FIFO. A full queue fails immediately; waiting and generation have
separate deadlines. Context metadata is re-read after admission, so a reload while
waiting cannot leave a stale capacity assumption. Streams use an eight-event
channel and keep admission through completion, errors or cancellation. A stalled
consumer cannot retain the generation slot past its deadline. Dropping the stream
closes the upstream response. Malformed/truncated EOF is an error, not success.

Regular chat calls honor their single-response token limit; they do not silently
generate additional replies. Use the explicit job runner for multi-step work.

Focused checks: `cargo test -p codetether-provider-openai`.
Explicit live probe (not proof of server deployment): set
`CODETETHER_LMSTUDIO_LIVE_BASE=http://127.0.0.1:18080/v1` and run
`cargo test -p codetether-provider-openai lmstudio_live_output_above_old_ceiling -- --ignored --nocapture`.