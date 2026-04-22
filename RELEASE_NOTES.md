# v4.6.0

## What's New

- **Derived Context Architecture** — Chat history is now cleanly separated from derived LLM context. A new `codetether context` CLI subcommand lets you inspect and manage context state directly. Session context goes through a dedicated derive → compress → reset pipeline with configurable policies. (#62)

- **RSS Watchdog & Memory Telemetry** — An RSS memory watchdog monitors process memory and triggers proactive context compression before OOM. The telemetry module (previously a single 812-line file) has been decomposed into focused sub-modules: token counters, tool execution tracking, provider metrics, swarm stats, cost guards, and persistent snapshots.

- **TUI `/autochat` Persona Relay** — The `/autochat` command now runs a real multi-step persona relay instead of a stub. Inline autochat persona and step-request modules drive multi-turn delegation with notification helpers and run summaries.

- **TUI Protocol View** — A new `/protocols` command and dedicated `ProtocolRegistryView` replace the old monolithic `protocol_registry.rs` (315 lines → 159-line focused view module). Navigation wired end-to-end.

- **Unified Search Router** — A new `src/search/` module provides a dispatch engine, typed results, request parsing, and an LLM-routed search tool (`search_router`) that picks the right backend (grep, glob, web, memory, RLM) per query.

- **Browser Networking Overhaul** — `browserctl` fetch/xhr replay now auto-inherits captured headers and provides JS-side fallback hints on failure. The 368-line `net.rs` has been split into ten focused modules: `fetch`, `axios`, `xhr`, `replay`, `log`, `diagnose`, each with its own template file.

- **Tool Compatibility Aliases** — A new `src/tool/alias.rs` system registers alternate names for tools, smoothing over provider-specific naming differences.

- **Session Internals Expanded** — New sub-modules for delegation & skills, evaluation, fault injection, history & history sink, journaling, oracle lookups, page-based context, relevance scoring, and a task subsystem (events, logs, state, rendering).

- **Provider Improvements** — Bedrock body builder and conversion split into dedicated modules. New `src/provider/pricing.rs` for cost estimation. GLM-5, Google, Vertex GLM, ZAI, and OpenAI Codex providers received upstream compatibility updates. LSP client and transport hardened.

## Bug Fixes

- **Streaming trim boundary corruption** — Fixed `trim_middle` in the streaming LLM helper so `cut_end` advances past orphaned `tool_calls` that would otherwise leave half-written function-call fragments at context boundaries.

- **RLM spin loops** — Narrowed the set of eligible tools available to the RLM router and added summary annotations so the recursive search terminates correctly instead of re-dispatching indefinitely. Added provider fallback in the search model layer.

- **TUI autochat & OKR approval wiring** — Fixed broken event flow between autochat relay steps and OKR approval dialogs.

## Changes

- **329 files changed** — 14,701 insertions, 13,271 deletions (net −570 lines with dramatically better modularity).
- **Top-level cleanup** — Removed 25+ `fix_*.py` scripts, 15 stale `prd_*.json` files, cached benchmark/error artifacts, `.vsix` extensions, and other clutter. Legacy documentation archived under `docs/legacy/`.
- **Ralph loop** — Refactored for cleaner PRD-driven iteration with improved progress tracking.
- **Swarm executor** — Streamlined subtask spawning; new `src/swarm/subtask.rs` isolates subtask definitions.
- **Install scripts** — `install.sh` and `install.ps1` updated to document new history S3 environment variables.
- **Config guardrails** — New `src/config/guardrails.rs` centralizes safety configuration.
