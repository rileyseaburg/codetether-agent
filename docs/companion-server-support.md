# Companion server support

## Scope and status

The companion supports user-consented Windows screen analysis streamed to the native Swift/SwiftUI iPhone app. It is not remote desktop control or tool-capable agent chat. The [companion contract](../windows/CodeTether.Companion/AGENTS.md) governs behavior; the [Rust migration notes](../windows/CodeTether.Companion/RUST_MIGRATION.md) record implementation boundaries.
**static/local:** A Rust relay now exists in [`crates/codetether-companion-relay`](../crates/codetether-companion-relay/) (HTTP routing, validation, owner/device authorization, capture requests, SSE, and direct streamed inference around the protocol and core crates). Its focused HTTP contract test and unit/doc tests passed locally against a synthetic analyzer; most TypeScript relay test files are not yet ported. **not-run:** Hosted inference, deployment, tunnel routing, and Windows/iOS end-to-end verification. **Live state:** the TypeScript relay (`codetether-companion.service`) is the deployed relay on `127.0.0.1:4099` behind the `/companion` tunnel route (see the companion delivery tickets in `windows/CodeTether.Companion/`); the Rust relay is not deployed.

## Architecture and hosting

```text
Windows selected-monitor capture -> authenticated relay -> hosted vision completion
iPhone session setup/questions   -> authenticated relay -> streamed analysis to iPhone
Hosted inference: https://server.codetether.run/v1/chat/completions
```

Keep the relay server-side, separate from the Windows application. Windows is one application in the existing Cargo workspace and must not spawn, embed, or supervise a CodeTether agent/server. Pairing alone does not select a capture scope: local pairing plus explicit monitor selection authorizes servicing owner requests without a separate Start step. Preserve visible sharing status and local Pause, Stop/unpair, lock/disconnect handling, and Exit.
Retain the existing relay process during migration, routing only `/companion/*` through the existing hosting boundary to its loopback listener (`127.0.0.1:4099`). Preserve existing API, audio, and tunnel routes. See [public-server operations](../scripts/public-server/README.md); this recommendation does not establish current live routing.

## Reuse and integration gaps

| Component | Responsibility and boundary |
| --- | --- |
| [Rust server](../src/server/mod.rs) | Existing OpenAI-compatible image input and streamed completions; use direct inference, not an agent loop. Accepts `reasoning_effort` and `service_tier` (see below). |
| [Rust relay](../crates/codetether-companion-relay/) | Loopback `/companion/*` relay binary (`codetether-companion-relay`); wire-compatible port of the TypeScript relay. |
| [TypeScript relay](../scripts/public-server/companion/) | Behavioral reference for HTTP, validation, capture requests, SSE, and analysis orchestration. |
| [Protocol crate](../crates/codetether-companion-protocol/) | Typed JSON contracts and shared wire fixtures, not a relay server. |
| [Core crate](../crates/codetether-companion-core/) | Memory-only owner/device authentication and session lifecycle, not transport, inference, or local capture permission. |

**static/local wiring gap:** [upstream.ts](../scripts/public-server/companion/upstream.ts) currently defaults to `http://127.0.0.1:4096`, whereas the contract requires the hosted HTTPS endpoint above. The Rust relay defaults to `https://server.codetether.run` (override with `CODETETHER_COMPANION_UPSTREAM`; non-loopback origins must be HTTPS and redirects are refused). Reconcile configuration and credential provisioning during integration; do not infer hosted wiring or successful provider access from the required design.
Use one memory-only registry per relay process behind a serialized mutation boundary, with trusted server time. Start with one relay instance; multiple replicas require deliberate session affinity or shared state/event design. Restart revokes sessions; do not silently restore device capabilities.

## API and authorization

Preserve exact status codes and JSON shapes from [routes.ts](../scripts/public-server/companion/routes.ts), [types.ts](../scripts/public-server/companion/types.ts), and [commands.ts](../scripts/public-server/companion/commands.ts). Paths below are relative to `/companion`.

| Caller | Routes | Authority |
| --- | --- | --- |
| Owner Bearer credential | `POST /sessions`, `GET /sessions/{id}/events`, `POST /sessions/{id}/request`, `POST /sessions/{id}/reply`, `DELETE /sessions/{id}` | Create, observe, request a fresh frame, queue one reply for the device to type, and stop. |
| Pairing code | `POST /pair` | Exchange a short-lived, single-use code for a session device token. |
| Device Bearer token | `POST /sessions/{id}/frames`, `POST /sessions/{id}/pause`, `POST /sessions/{id}/typed`, `GET /sessions/{id}/commands` | Upload, pause, acknowledge a typed reply, and poll only the paired session. |

Device tokens must never authorize owner or general API operations. Keep owner inference credentials server-side through the established Vault/environment boundary. Preserve hashed token storage, constant-time verification, origin checks, HTTPS and redirect rejection; never log credentials, prompts, or screenshots. If later embedding the relay in the Rust server, preserve mandatory API authentication and explicitly design companion authorization and policy mappings; do not exempt the entire prefix or treat device tokens as general API credentials.

## Lifecycle, limits, and privacy

- Creation requires `provider/model`, a nonblank prompt of at most 2,000 characters, and an integer interval of 15–300 seconds. Pairing codes are twelve hexadecimal characters, single-use, and valid for five minutes. Sessions expire after one hour; retain four-session and thirty-pairing-attempts/minute registry bounds.
- Frames use `image`, `captured_at`, and optional `trigger`/`request_id`. Preserve 710,000 request bytes, 700,000 image characters, 512 KiB decoded JPEG, JPEG validation, maximum 1920×1920 dimensions, freshness checks, and upload/session-change conflict handling.
- Allow one active analysis and 120 accepted captures per session. Preserve five-second nonperiodic cooldowns, configured periodic intervals, coalescing/backpressure, and bounded work rather than an unlimited screenshot queue.
- Owner questions require a matching fresh frame within sixty seconds. Devices receive only an opaque request ID, never the question. Remote requests cannot resume capture or widen the selected monitor scope.
- Owner replies are the only owner text a device receives: `POST /sessions/{id}/reply` queues one reply of 1–2,000 characters (409 when unpaired or one is already queued); devices re-receive it in `GET /commands` until they ack `POST /sessions/{id}/typed`, and undelivered replies expire after sixty seconds with an error event. Pause and Stop clear queued replies. Explicit owner typing requests (beginning `Type`, `Enter`, `Write` or `Fill`, with optional polite prefixes) also authorize a direct one-shot handoff after their matching fresh analysis succeeds: both relays parse the final `windows-reply` JSON block, queue only its bounded single-line text, and replace it with queued/not-queued status in the final analysis. There is no secondary iPhone review; periodic captures, partial/error output, and reconnect snapshots never trigger this handoff. The descriptive target is not a selector; queueing and ACK consumption do not prove insertion. Direct handoff verification (builds/checks/tests/deployment/device use): **not-run**. Typing stays local to the Windows app: keyboard-only insertion into the focused editable input, gated by local pairing and monitor selection, halted by Pause/Stop/unpair.
- Preserve SSE `snapshot`, `capture`, `delta`, `done`, `error`, and `stopped`, sequence numbers, three-viewer limits, heartbeats, and stale-work cancellation. Reconnect sends a current snapshot, not duplicate delta replay. Disable proxy buffering and choose streaming timeouts deliberately.
- Inference must be direct, streamed, bounded, and use `tools: []` with an explicitly selected vision-capable model. Treat screenshot text and previous analysis as untrusted data; never execute them or repeat credentials. Report actual requested/resolved models and concrete diagnostics without guessing availability.
- Stop revokes pairing, aborts analysis, clears pending/current/previous analysis, and closes streams. Keep images and device credentials memory-only; do not add screenshot persistence, image endpoints, or iPhone pixel display. Provider retention is separate and cannot be described as guaranteed deletion.

## Fast tiers and thinking levels

`POST /v1/chat/completions` accepts two optional OpenAI-style fields, currently honored only for the `openai-codex` provider:

| Field | Values | Effect |
| --- | --- | --- |
| `service_tier` | `fast`/`priority`, `ultrafast`, `default`/`auto`/`flex` | Selects the Codex priority or ultrafast tier. |
| `reasoning_effort` | Levels the model advertises (for example `low`, `high`, `xhigh`, `max`, `ultra`) | Sets the thinking effort. |

They are equivalent to model suffixes (`gpt-6-astra-ultrafast:high`). A tier or effort the selected model does not advertise, or either field on another provider, returns 400. `GET /v1/models` lists `service_tiers` and `reasoning_efforts` per model and adds the fields to `supported_parameters`. Streamed reasoning is emitted as `delta.reasoning_content`. **not-run:** these server changes have not been compiled or tested.

## Relay configuration

`codetether-companion-relay` reads `CODETETHER_AUTH_TOKEN` (owner credential, ≥32 chars), `CODETETHER_COMPANION_UPSTREAM`, `CODETETHER_COMPANION_ORIGIN`, `CODETETHER_COMPANION_ADDR` (default `127.0.0.1:4099`), and optional `CODETETHER_COMPANION_ASSETS` (web shell directory). SIGTERM or Ctrl-C stops every session before exit. There is no systemd unit or deploy script for it yet; `deploy.sh` still installs the TypeScript relay.

## Migration sequence and evidence

1. Implement small Rust HTTP, validation, authentication, capture-request, SSE, and inference modules around the existing libraries; preserve wire compatibility and the owner/device split.
2. Reconcile hosted inference wiring, bounded streaming, cancellation, and proxy behavior while retaining the existing relay until parity evidence supports cutover.
3. When verification is explicitly requested, record focused contract/authentication/limits/reconnect checks separately from real hosted inference and Windows/iOS end-to-end evidence. Preserve artifacts and rollback without duplicate capture agents or restored revoked sessions.

**Evidence boundary:** This document is an architecture proposal grounded in **static/local** source inspection, not implementation, packaging, release, or live deployment evidence. Documentation checks do not establish runtime behavior.