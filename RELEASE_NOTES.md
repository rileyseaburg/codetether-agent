# v3.0.0

## What's New

- **Glob tool improvements**: The `glob` tool now respects `.gitignore` patterns and supports configurable depth limits using `WalkBuilder`, providing more precise and efficient file discovery

## Changes

- Enhanced `glob` functionality in `src/tool/file.rs` with WalkBuilder integration for better directory traversal
- Minor TUI cleanup in `src/tui/mod.rs`

**Stats**: 2 files changed, 39 insertions(+), 5 deletions(-)
