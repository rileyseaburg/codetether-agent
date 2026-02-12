# v1.1.7

## What's New

- **Easy-mode slash commands**: Added convenient slash commands in the TUI for streamlined interactions
- **Improved autochat UX**: Enhanced the autochat experience with better interface and workflow

## Bug Fixes

- Fixed PowerShell compatibility by replacing ternary operators with if/else statements in `install.ps1`, ensuring support for older PowerShell versions

## Changes

**2 files changed**, 251 insertions(+), 57 deletions(-)

| File | Changes |
|------|---------|
| `install.ps1` | PowerShell compatibility improvements |
| `src/tui/mod.rs` | Slash commands and autochat enhancements |
