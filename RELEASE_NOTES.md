# v4.7.0-a-002.2

## What's New

- Improved the release publishing workflow to handle workspace crate ordering more reliably.

## Bug Fixes

- Fixed the publish sequence so workspace crates are published before the main `codetether-agent` crate, preventing dependency availability issues during release.

## Changes

- Updated workspace package metadata for browser and RLM crates.
- Refined `release.sh` with expanded release automation logic.
- Adjusted root `Cargo.toml` release configuration for v4.7.0-a-002.2.
