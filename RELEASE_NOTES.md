# v4.6.4

## What's New

- **Improved SSH image paste experience** — Pasting images over SSH connections now shows guided error messages and links to relevant help documentation, making it much easier to diagnose and fix clipboard issues in remote sessions (#97).
- **Release asset backfill workflow** — Added a GitHub Actions workflow to backfill release assets, ensuring historical releases have complete downloadable binaries.

## Bug Fixes

- **Default workers now use Codex GPT-5.5** — Worker model defaults were updated to Codex GPT-5.5, ensuring new and existing workers pick the correct model without manual configuration.
- **macOS runner for release backfill** — Switched to an available macOS runner to fix the release asset backfill workflow.

## Changes

- Alphabetized imports and reformatted assertions across `worker.rs`, `tool/mod.rs`, `model_defaults.rs`, and `client.rs` for consistency.
- Streamlined model preference logic in `worker/model_preferences.rs`, reducing complexity and removing redundant codepaths.
- Extracted SSH clipboard handling into a dedicated `clipboard_ssh.rs` module for better separation of concerns.
- Updated model references across session helpers, the smart switch, and the swarm orchestrator to align with the new defaults.
- Expanded TUI help documentation to cover SSH clipboard workflows.

**Full diff**: 24 files changed, 334 insertions, 174 deletions.
