# v4.7.0-a-002.3

## What's New

- Added run checkpoint auto-resume support, including richer checkpoint extraction from session messages and clearer step-limit semantics.
- Expanded Windows `computer_use` capabilities with client-area coordinates, window-relative input, staged mouse primitives, configurable drag behavior, and improved cursor handling.
- Added TetherScript model-provider improvements, including provider-name injection in `list_models` output and new example probes for OpenAI Codex image workflows.
- Added worker A2A peer seed support and mDNS peer startup for improved distributed worker discovery.
- Introduced memory scoping support for more predictable project-aware memory behavior.

## Bug Fixes

- Fixed Windows `ClientToScreen` handling and imports for more reliable screen-coordinate conversion.
- Hardened Windows computer-use input handling, including mouse, text, scroll, click, double-click, right-click, and drag flows.
- Fixed session task and worker task persistence so UI and worker state changes are retained.
- Fixed stale Git config lock recovery.
- Prevented voice transcription state leaks.
- Improved RLM context management, fallback behavior, and quality gates.
- Fixed checkpoint assistant handling with exhaustive `ContentPart` matching and split checkpoint state extraction.
- Tolerated already-published Jenkins releases during release automation.
- Accepted `CodeTether-Worker-ID` as provenance in `verify_pr.sh`.
- Removed cached `sessions_dir()` state to preserve test isolation when `CODETETHER_DATA_DIR` changes.

## Changes

- Refactored `computer_use`, memory, RLM, session checkpointing, session task handling, and session recall internals into smaller SRP-focused modules.
- Separated Knative and persistent worker capabilities.
- Updated validation evidence rules, workflow evidence templates, and prompt tests for clearer validation terminology.
- Improved documentation for mDNS service handling and lifecycle management.
- Updated TetherScript dependency documentation and examples.
- Adjusted CI/release files, including Jenkins, Makefile, and Windows Docker release configuration.
- Release diff summary: 151 files changed, 3,732 insertions, 1,093 deletions.
