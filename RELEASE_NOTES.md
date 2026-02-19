# v3.2.2

## What's New

- **Worker Workspace Auto-Sync**: A2A workers now automatically sync workspace state, ensuring consistent codebase tracking across worker sessions
- **Interactive Auth Blocking**: The bash tool now blocks interactive authentication prompts, preventing runaway terminal sessions and improving automation reliability
- **TUI Improvements**: Significant enhancements to the terminal UI for better responsiveness and user experience

## Changes

- Extended A2A worker module with workspace synchronization capabilities (+136 lines)
- Enhanced bash tool with interactive prompt detection and blocking (+121 lines)
- Updated provider layer with additional authentication handling
- Refined server request processing
- Improved TUI rendering and state management
- Minor sandbox policy updates for new bash behavior

**Stats**: 6 files changed, 401 insertions(+), 58 deletions(-)
