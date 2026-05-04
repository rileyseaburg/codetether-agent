# v4.7.1

Here are the release notes for **v4.7.1**:

---

## What's New

- **Windows native desktop automation** — The `computer_use` tool now uses Win32 APIs directly via the `windows` crate for screen capture (GDI), input simulation (click, drag, scroll, key chords, text entry), window management, and process discovery, replacing the previous PowerShell-based approach with significantly faster, more reliable desktop interaction.
- **Tetherscript-backed Cerebras provider** — New LLM provider integration using Tetherscript plugins, enabling Cerebras inference through the standard provider interface with streaming support.
- **DeepSeek repair pipeline** — Automated repair flow for DeepSeek provider responses, including fixes for tool-calling body construction, conversion, streaming, and model limits.
- **Worker task lifecycle endpoints** — Three new server endpoints for A2A worker task management: `POST /worker/task/stream` (assign tasks), `POST /worker/task/claim` (claim pending tasks), and `POST /worker/task/release` (release task ownership), enabling more flexible distributed task routing.
- **TUI UX enhancements** — Eight improvements including tool panel auto-follow (tracks latest tool execution), inline diff display in the tool panel, latency badges per chat turn, smarter slash-command suggestions, and improved PR title/body generation from worktrees.
- **Shadow DOM support** — Browser automation (`browserctl`) now pierces shadow boundaries for element queries, uploads, and interactions using the `>>>` combinator.
- **RLM router improvements** — The Recursive Language Model router now supports richer query classification and delegation across search backends.
- **Audit worktree isolation and sandboxing** — New audit logging for worktree creation, sandbox policy enforcement, and isolation gating to ensure swarm sub-agents operate within their designated boundaries.
- **Session recall budget awareness** — `session_recall` now uses budget-aware context flattening to prevent OOM conditions when reconstructing large conversation histories.
- **Vendored Chromiumoxide for Windows** — Bundled `chromiumoxide` crates under `vendor/` for reliable Windows `computer_use` browser automation without external dependency resolution.

## Bug Fixes

- **Worker task routing** — Workspace IDs are now correctly resolved during worker registration, fixing server-side task routing that previously mismatched workers to codebases.
- **Worker poll filtering** — `poll_pending_tasks` now skips tasks targeted at a different agent, preventing cross-agent task stealing in multi-agent deployments.
- **Cerebras rate-limit retry** — Cerebras HTTP 429 responses are now classified as retryable errors, enabling automatic backoff and retry.
- **False-positive retries** — HTTP 200 OK responses are no longer incorrectly retried; the retry classifier now checks status codes before inspecting error bodies.
- **Tetherscript streaming fallback** — When Tetherscript streaming fails, the provider emits a single-chunk stream containing the full response instead of returning a hard error.
- **Context compaction task preservation** — Active tasks are now preserved across context compaction events, preventing task loss during long sessions.
- **`floor_char_boundary` stability** — Replaced the unstable `floor_char_boundary` API with a stable char-boundary loop for broader Rust version compatibility.
- **Docker build fixes** — Added system protobuf include paths for Linux Docker builds, corrected `COPY` directives in the Windows Dockerfile, and removed `examples` from `.dockerignore` to ensure example files are included in build context.
- **Unix-only `stderr_guard`** — `stderr_guard.rs` now gates Unix-specific APIs behind `cfg(target_os = "linux")`, fixing compilation on Windows targets.
- **Windows crate 0.62 migration** — Updated all Windows platform code for API changes in the `windows` crate 0.62 release.
- **Tetherscript version** — Pinned `tetherscript` dependency to crates.io `0.1.0-alpha.7` for stable builds.
- **Clipboard on Windows** — New WinAPI-backed clipboard implementation replaces previous approach, resolving encoding and reliability issues.

## Changes

- **Swarm validation refactored** — The monolithic `swarm/validation.rs` (734 lines) decomposed into 26 focused modules under `src/swarm/validation/`, each under 50 lines, following SRP and modular cohesion guidelines.
- **Tool panel SRP refactor** — `src/tui/ui/tool_panel.rs` (403 lines) split into 18 dedicated modules covering panel chrome, diff rendering, item builders, previews, and formatting.
- **Session helper modularization** — `prompt_too_long.rs`, `session_recall`, `recall_context`, and `tetherscript_repair` each refactored into focused submodules for testability and readability.
- **Image tool overhaul** — `src/tool/image.rs` rewritten with proper MIME detection, base64 encoding, and URL handling for vision model integration.
- **Worker registration refactored** — A2A worker split into dedicated modules: `task_timeline.rs` for instrumentation, `workspace_resolve.rs` for ID resolution, and streamlined `worker.rs` core.
- **Delegation lambda** — New `session/delegation/lambda.rs` for lightweight, one-shot task delegation with outcome tracking.
- **Eval module restructured** — `session/eval.rs` promoted to `session/eval/mod.rs` with `eval_default_flip.rs` extracted as a focused sub-module.
- **Active tail tracking** — New `session/context/active_tail.rs` for monitoring the tail of active conversation context, with dedicated tests.
- **449 files changed** across platform abstractions, providers, TUI, swarm orchestration, and tooling — **151,709 insertions**, **2,384 deletions**.
