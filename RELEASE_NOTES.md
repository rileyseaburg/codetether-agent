# v4.7.1

## What's New

- Added MiniMax-M3 support in the provider model picker, including a 1M context window limit and default model selection updates.
- Improved TUI responsiveness with dirty-state redraw tracking, reduced unnecessary redraws, zero-clone message caching, and lower tick overhead.
- Added live TUI tool progress updates so long-running tool activity is easier to follow.
- Enhanced the file picker with Home/End navigation, virtual scrolling, preview caching, and a built-in file browser viewer.
- Added runaway tool-call guardrails to reduce model spinning and uncontrolled tool loops.
- Added Blender-focused helpers to `computer_use`, including selection, viewport framing, and visual evidence support.
- Added TetherScript computer capability grants, allowing plugins to use scoped desktop automation capabilities.
- Improved A2A worker behavior with session persistence fixes, peer liveness support, and TUI heartbeat updates.
- Added frozen-prefix cache helpers for smoother streaming chat rendering.
- Added a lean tool profile and headless auto-apply mode for benchmarking workflows.

## Bug Fixes

- Fixed file picker attachment behavior to skip directory preview paths.
- Fixed MiniMax-M3 model limit matching and addressed related review feedback.
- Fixed invalid `argv` handling in allocation guard reports.
- Added a capacity-guarding global allocator to prevent OOM aborts.
- Fixed release compile blockers and Windows cross-compilation errors.
- Fixed worker test provider dependency imports.
- Restored the worker release module.
- Hardened worker git and Blender actions.
- Addressed PR review feedback and ratchet compliance issues.

## Changes

- Decomposed the A2A worker monolith into a modular SRP-oriented module tree.
- Split tool, worker, TUI event loop, Blender input, edit, provenance, and TetherScript runner logic into smaller focused modules.
- Refactored provenance git and hook handling, including commit argument handling, git identity, hook path/script/chmod/seed helpers, and trailer management.
- Added configurable commit signing policy through `CODETETHER_GIT_COMMIT_SIGN`.
- Pre-seeded provenance trailers through `ensure_provenance_trailers`.
- Added `.cargo/config.toml` with linker flags and SQLX offline mode.
- Updated workflow runners to use `codetether-agent-k8s` self-hosted infrastructure.
- Added and expanded tests for MiniMax metadata/context limits, TetherScript computer grants, file picker navigation, TUI tool status events, worker behavior, and peer liveness.
