# v4.5.7

## What's New

- **XHR replay in browserctl** — New `xhr` action replays requests as raw `XMLHttpRequest`, preserving `Sec-Fetch-*` headers and cookie semantics for sites where `fetch`/`axios` replay fails.
- **Network introspection suite** — Added `axios` (replay via the page's own axios instance with inherited interceptors/baseURL) and `diagnose` (dumps service workers, axios instances, CSP headers, and network log summary) to browserctl.
- **TUI paste-burst heuristic** — Clipboard paste events are now detected and batched to avoid flooding the input handler with individual character events.
- **Native text selection in TUI** — Mouse capture is disabled, allowing users to select and copy terminal text with the mouse without interfering with the TUI.
- **Steering cancel** — Mid-generation steering can now be cancelled cleanly using `notify_one`, preventing spurious wakeups.
- **Multi-line Enter** — The TUI input now supports multi-line entry via Enter key with proper submit handling.
- **Browser session lifecycle** — New attach-to-existing, discovery, and enhanced launch flows for browser sessions, including device emulation and DOM inspection helpers.
- **Session tail-loading** — Sessions now support tail-loading and tail-seeding for faster startup with large conversation histories.
- **Shared HTTP provider utilities** — Common HTTP plumbing extracted into `shared_http.rs` and `body_cap.rs` to reduce duplication across LLM providers.
- **Dedicated shell tool** — New `bash_shell` tool provides isolated shell execution separate from the general `bash` tool.

## Bug Fixes

- **Steering cancel correctness** — Fixed steering cancellation to use `notify_one` instead of `notify_all`, eliminating unnecessary wakeups of unrelated tasks.
- **Worker bridge stability** — Hardened the TUI ↔ worker bridge to handle edge cases in event propagation.

## Changes

- **Session persistence refactor** — Large rewrite of session persistence (`+431` lines) improving serialization, compression, and workspace indexing.
- **Browser wait utilities** — New `wait.rs` module (`+342` lines) with composable wait strategies for browser automation.
- **Browser network interception** — New `session/net.rs` (`+368` lines) enabling request interception and modification.
- **RLM oracle validator** — Minor updates to the Recursive Language Model's oracle validator pipeline.
- **Provider consistency** — Standardized request handling across Anthropic, Bedrock, OpenAI, OpenRouter, Copilot, GLM5, Google, Moonshot, StepFun, and ZAI providers.
- **111 files changed**, 5,158 additions, 366 deletions across browser, TUI, session, provider, and tool modules.
