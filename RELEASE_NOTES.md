# v4.7.0-a-002.1

## What's New

- Added a standalone `codetether-rlm` crate, extracting RLM routing, chunking, context tracing, oracle logic, model selection, and event handling into a dedicated reusable package.
- Introduced a standalone `codetether-browser` crate and replaced the BrowserCtl Chromium backend with a TetherScript-powered browser capability backend.
- Added BrowserCtl offline diagnostics for auth traces, cookie diffs, CORS analysis, replay records, and network inspection.
- Added `/git` TUI view with branch status, log, diff, and branch list details.
- Added TUI context health surfacing, including session context status and compaction visibility.
- Added background RLM context processing and parallel TUI tool execution support.
- Added runtime validation, evidence, and memory-acceleration guardrails to improve task accountability and proof tracking.
- Added task scope gating to prevent agents from acting on out-of-scope worker tasks.
- Added thinking configuration controls with request-specific disable logic.
- Upgraded TetherScript integration to `v0.1.0-alpha.10` and added browser authority support for live browser capabilities.

## Bug Fixes

- Hardened TUI image attachment handling.
- Fixed Docker release builds by updating builder images to Rust 1.89.
- Fixed bounded truncation accounting for retained event and chat payloads.
- Fixed TUI remove/undo command alias handling and related review feedback.
- Fixed public RLM API usage for oracle test result imports.
- Fixed async-state handling so TetherScript page wrappers are not retained incorrectly.
- Fixed native browser helper exports across module boundaries.
- Fixed parallel tool scheduler entrypoint visibility.
- Fixed context summary wiring for RLM compaction.
- Fixed stable character-boundary handling for summaries.
- Preserved summary residency during incremental context repair.
- Fixed worker context recovery with compact tool schemas.
- Bounded worker grep execution and retained terminal payloads to reduce runaway memory or output growth.
- Fixed sub-agent persistence, serialization, token exhaustion handling, and parent workspace preservation.
- Fixed worker task polling so pending tasks are filtered by agent target.
- Fixed bus, TUI, and A2A integration wiring.
- Fixed clipboard linkage across Unix and Windows fallback paths.
- Tightened streaming, session, and thinker memory allocation caps plus RSS watchdog behavior.

## Changes

- Applied broad `cargo fmt` cleanup across RLM chunker, router, TUI, and session modules.
- Refactored large RLM, browser, TUI session event, context, and tree-sitter oracle modules into smaller SRP-focused modules aligned with the 50-line file ratchet.
- Split browser request, session, native interaction, network, tab, DOM, and lifecycle logic into modular components.
- Split context summarization, compression, validation, evidence, refactor guard, RLM background, and parallel tool code into focused session helper modules.
- Added new documentation for BrowserCtl TetherScript backend, browser capability APIs, offline BrowserCtl usage, TetherScript dependency guidance, and CI Docker toolchain policy.
- Promoted TetherScript plugin platform documentation in the README.
- Limited CodeTether GitHub Action execution to PR workflows.
- Kept CodeTether git credential helper behavior out of worktrees and ensured helper stdout stays clean.
- Removed an unused image format re-export.
- Added regression and integration coverage for swarm token limits, context health, image input handling, turn undo/remove, TetherScript browser grants, and RLM/context behavior.
