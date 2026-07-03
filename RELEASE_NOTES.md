# v4.7.3

## What's New

- **QUIC transport (Phase 6)** — Multiplexed QUIC stream transport with connection migration and framed session resume, plus a TUI transport widget and TCP_INFO telemetry sampling.
- **Bedrock thinking-effort & service-tier settings** — New TUI Settings controls for Claude Fable 5 / Opus adaptive reasoning, backed by thread-safe process state instead of runtime env-var mutation. `claude-fable-5` now routes through the Bedrock InvokeModel adapter, with mid-session bearer-token refresh on live auth failure.
- **Interactive `/spawn` form** — Wired into TUI render and key dispatch, with non-blocking detach mode for sub-agent messaging.
- **Cerebras model support** — Registered `gemma-4-31b` in the default catalog and capture `reasoning_content` via raw SSE stream to prevent empty turns.
- **Workspace-aware release publishing** — `release.sh` now checks crates.io for existing versions and treats already-published workspace crates as a skippable, non-fatal case.

## Bug Fixes

- **Sub-agent workflows (#294–#297)** — Broke an infinite identical-edit retry loop, auto-start the first turn on spawn, unified sub-agent registries for TUI visibility, made dispatch non-blocking by default, surfaced sub-agent liveness from tool activity, and enabled auto-apply for spawned sub-agents.
- **Worktree safety (#297)** — Guard worktree removal against a dirty working tree so uncommitted changes are never discarded.
- **OKR progress** — Zero-target key results now report correct progress and completion.
- **OpenAI-compatible streams** — Repair dangling tool-call sequences that could break streaming turns.
- **Vector DB** — Make BERT embeddings contiguous and log embedding failures instead of silently failing.
- **Memory** — Always run init so the embedder installs on every tool instance.

## Changes

- Refactored oversized TUI settings modules to meet the 50-line file budget.
- Raised the Fable default thinking effort to medium and require narration before tool calls.

_181 commits since v4.7.1 · 1,671 files changed._
