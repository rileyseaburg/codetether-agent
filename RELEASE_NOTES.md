# v1.1.8

## What's New

No new features in this release.

## Bug Fixes

- **ZAI Provider**: Fixed tool call argument serialization to properly convert arguments to JSON strings before sending to the ZAI API. This resolves issues where tool calls with complex argument structures were failing.

## Changes

- Updated ZAI provider implementation with improved argument handling
- 1 file changed: `src/provider/zai.rs` (+55/-5)
