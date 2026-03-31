# v4.4.0

## What's New

- **Codex Session Import** — Import sessions from OpenAI Codex CLI via `--import-codex` flag or TUI command, with resilient session listing
- **Smart Model Switching** — Automatic fallback to alternative models on API errors with configurable retry strategies
- **Watchdog Timer** — Detect and recover from stuck LLM requests with configurable timeouts
- **Spawned Sub-Agent System** — New `/spawn`, `/kill`, and `/agents` commands for managing sub-agent lifecycles with isolated state
- **OKR Approval Gate** — Interactive `/go` command for OKR-driven autonomous execution with approval workflows
- **Undo Command** — `/undo` command to revert recent AI-generated changes via git
- **Easy-Mode Aliases** — Simplified command aliases for common workflows
- **GLM-5 Turbo Model** — Support for ZhipuAI's GLM-5 Turbo via the `zai` provider
- **Pony Alpha 2 Model** — New model support with TUI bracketed paste mode
- **GitHub PR Integration** — New `github_pr` module for automated PR creation and management
- **Provenance System** — Git identity, signatures, and audit trail for tracking code origins
- **CloudEvents Support** — Structured event emission for external integrations
- **TUI Tool Panel** — Dedicated panel for tool execution visibility
- **Latency Tracking** — Request latency monitoring and display in TUI

## Bug Fixes

- Fixed Windows cross-compilation by gating Unix-only `chmod` behind `cfg(unix)`
- Fixed merge conflict cascade failures with automatic `-X theirs` resolution strategy
- Fixed worktree merge failures by stashing dirty working trees before merge operations
- Fixed TUI message scrolling and multi-line input layout issues
- Fixed post-edit validation enforcement in `run` and `serve` commands
- Fixed missing `ViewMode::Protocol` match arm causing compile errors
- Fixed missing state fields and imports in AppState for smart switching
- Addressed PR #11 review feedback

## Changes

- **A2A Worker** — Enhanced worker registration, claim handling, and credential management
- **Ralph Loop** — Improved autonomous PRD-driven development with better worktree isolation
- **Session Management** — Refactored listing, import, and persistence handling
- **Forage System** — Mission-aligned autonomous work selection with moonshot rubric scoring
- **Vendor Dependencies** — Bundled `candle-core` tensor library for local inference support
- **Documentation** — Added A2A Worker Integration handoff, Codex CLI MCP configuration guide, and TUI port checklist
- **Provider Registry** — Expanded provider configurations and error handling

---

**Stats:** 232 files changed, 57,450 insertions(+), 909 deletions(-)
