# v4.1.2

## What's New

- **Git Credential Helper and Worktree Management Infrastructure**: Added comprehensive Git credential helper (`src/a2a/git_credentials.rs`) and worktree management system (`src/worktree/manager.rs`, `src/worktree/helpers.rs`, `src/worktree/cleanup.rs`, `src/worktree/integrity.rs`, `src/worktree/merge.rs`) for improved codebase isolation and management.
- **Symbol Search View**: Introduced a new symbol search feature (`src/tui/symbol_search.rs`) for navigating codebases more efficiently in the TUI.
- **Task Management CLI**: Added dedicated task commands (`src/cli/task.rs`) for creating and managing sub-tasks.

## Bug Fixes

- **Watchdog Timeout Improvements**: Fixed aggressive watchdog behavior by increasing timeout thresholds (90s → 180s → 300s) and raising minimum clamp to 60s.
- **Infinite Retry Loop**: Resolved issue where watchdog never marked requests as failed, causing infinite retries.
- **Morph Backend Configuration**: Made Morph backend opt-in rather than default, giving users more control over tool execution.
- **Commit Message Generation**: Improved `commit.sh` with better message generation and context-aware fallbacks.

## Changes

- **API Refinements**: Renamed `matched_line` field in `codesearch` results for clarity.
- **Configuration Updates**: Updated `Cargo.toml` and `Cargo.lock` dependencies.
- **Provider Adjustments**: Minor updates to Bedrock provider implementation.
- **Tool Enhancements**: Refined `edit`, `morph_backend`, and `multiedit` tool implementations.
- **Test Updates**: Improved `tests/morph_tool_integration.rs` test coverage.
