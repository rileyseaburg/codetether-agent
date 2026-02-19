# v3.3.0

## What's New

- **Completion callbacks for Go tool** — The autonomous execution pipeline now supports real-time completion callbacks, enabling immediate notifications when tasks finish. This allows downstream systems to react instantly to pipeline completion events without polling.

## Changes

- Enhanced `go` tool with callback registration and invocation (50+ lines of new functionality)
- Refactored A2A worker to integrate with the new callback system
- Minor server routing updates to support callback endpoints

**Stats:** 3 files changed, 87 insertions(+), 24 deletions(−)
