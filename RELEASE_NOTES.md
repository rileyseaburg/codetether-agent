# v4.4.1

## What's New

- **Webview Chat Layout Mode** — New TUI layout with a sidebar, header, and status bar for an IDE-like chat experience (#US-001).
- **Agent Identity System** — Named profiles, fallback avatars, and formatting for distinguishable agent personas in the TUI (#US-002).
- **Inspector Panel** — Real-time tool-call history, layout, and metrics overlay for debugging agent sessions (#US-003).
- **Clipboard Image Support** — Paste images directly into the TUI, with SSH/headless fallback handling (#US-005).
- **Smart Model Switch** — Automatic detection of provider errors and seamless retry with an alternate model (#US-006).
- **Watchdog Stalled Request Recovery** — Background detector identifies hung requests and notifies the user with recovery options (#US-007).
- **Autochat Relay Integration** — Inter-agent relay communication for task delegation and result aggregation within the TUI (#US-004).
- **Kubernetes Tool** — Full K8s resource management (pod listing, scaling, rolling restarts, sub-agent pod spawning) via the `kubernetes` tool.
- **Post-Clone Follow-Up Tasks** — Worker enqueues follow-up tasks automatically after repository cloning.
- **Copilot Custom Agent Profile** — New `.github/agents/codetether.agent.md` profile for GitHub Copilot integration.
- **`--max-steps` CLI Flag** — Configurable step limit for agent runs.

## Bug Fixes

- **Bedrock Opus 4.7 compatibility** — Removed deprecated `temperature` parameter for Claude Opus 4.7 on AWS Bedrock (#50).
- **Worktree branch leak** — Branches are now deleted after worktree cleanup, preventing accumulation (#33).
- **Z.AI provider stability** — Fixed tool-call argument serialization (JSON object vs. string), routing, scalar argument unwrapping, and added retry logic for transient errors.
- **TUI session recording** — Sessions are now correctly persisted to disk; fixed stuck-processing state and missing auto-scroll on streaming text (#15, #19).
- **Sub-agent session persistence** — Sub-agent sessions are saved to disk instead of being lost on exit.
- **Truncated tool call recovery** — Detects and gracefully recovers from truncated tool call arguments.
- **Server dispatch** — Improved error handling and truncated description payloads to respect server `max_length` validation.
- **`floor_char_boundary` instability** — Replaced with a manual char-boundary search to avoid nightly-only API.

## Changes

- **SRP refactor of `agent` tool** — Split the monolithic `src/tool/agent.rs` (537 lines) into 25 focused submodules covering policy, spawning, event loops, sessions, and schema.
- **SRP refactor of `a2a/worker`** — Decomposed the 449-line worker into 17 modules for auth, runtime, metadata, forage settings, and workspace management.
- **SRP refactor of `server/auth`** — Broke the 238-line auth module into claims, middleware, token handling, and state submodules.
- **SRP refactor of `git_credentials`** — Split into 11 modules (gh_cli, gh_query, git_config, material, script, etc.).
- **SRP refactor of TUI modules** — Decomposed `event_handlers`, `event_loop`, `input`, and `smart_switch` into 40+ focused submodules.
- **Retry infrastructure** — New `src/provider/retry/` module with error classification, configurable retry policy, and streaming-aware retry logic.
- **Oracle RLM modularization** — Restructured `src/rlm/oracle/` into storage, consensus, batch, validation, and trace submodules.
- **Bash tool hardening** — Updated `src/tool/bash.rs` with improved sandbox constraints.
- **Comprehensive rustdoc standards** — Added documentation guidelines to `AGENTS.md` and fixed all broken doc tests across the crate.
- **README cleanup** — Removed 41 duplicate Codex CLI blocks and restructured for clarity.
- **CI updates** — Bumped `actions/create-github-app-token` v2 → v3.1.1; fixed `npm-release` workflow dispatch and secrets handling.
- **Removed tracked `.codetether-agent/` data** — Purged ~190K lines of local state files (sessions, indexes, memory, telemetry) from git tracking.

**452 files changed · 22,071 insertions · 226,113 deletions**
