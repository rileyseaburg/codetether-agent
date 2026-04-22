# v4.6.1

## What's New

- **Session history file management** — New `history_files.rs` module provides dedicated file-level operations for session history, improving separation of concerns.
- **Request state tracking** — Added `request_state.rs` to session helpers for structured request lifecycle management.
- **Prompt event pipeline refactor** — Rewrote `prompt_events.rs` (251 lines changed) for cleaner event-driven prompt assembly.
- **Context retention policy improvements** — Enhanced context policy (`policy.rs`, `derive.rs`, `helpers.rs`) with more granular control over what session context is preserved across resets.
- **Context reset rebuild hooks** — `reset_rebuild.rs` and `reset_helpers/` now support richer post-reset reconstruction of working context.
- **Installer improvements** — Updated PowerShell (`install.ps1`) and npm installer (`installer.js`) with better macOS and Windows detection and error handling.
- **Tool deduplication enhancements** — Improved `dedup.rs` and `tool_call_dedup.rs` for smarter duplicate tool call detection during sessions.

## Bug Fixes

- **Restored session context retention wiring** (`2d95505`) — Context retention was not correctly wired through the reset pipeline, causing session state to be lost after context resets.
- **PR review feedback addressed** (`f8f81ba`) — Minor fixes across session context and helper modules based on code review.

## Changes

- **README updated** with new CLI commands, tool renames, and improved platform-specific installation docs.
- **`context_browse` tool** refactored (218 lines changed) for simplified turn listing and retrieval.
- **`session_recall` tool** extended with additional metadata and improved recall accuracy.
- **TUI** gained new commands and improved message text rendering.
- **Telemetry watchdog** (`rss_watchdog.rs`) tuned for more accurate memory monitoring.
- **Session journal** now records delegation events and relevance scoring.
- **Codex import parser** received a minor fix for edge-case input.
- **RLM module** and **search model** received minor internal cleanups.

*46 files changed, 1,412 insertions(+), 411 deletions(-)*
