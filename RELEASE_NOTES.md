# v3.1.0

## What's New

### Persistent Ralph State Store
Added a new state store system for Ralph autonomous agent execution with multiple backend implementations:
- **In-memory store** for ephemeral runs
- **HTTP-backed store** for distributed state management
- Ralph loops now persist progress across iterations, enabling better recovery and monitoring

### Background Go Execution
The `go` command now supports background execution mode for long-running autonomous tasks:
- Execute OKR → PRD → Ralph pipelines asynchronously
- Watch pipeline progress with the `watch` action
- Check final status by OKR ID

### MCP Server Configuration Guide
Added comprehensive documentation for configuring MCP servers with:
- VS Code integration
- Claude Desktop setup

### Worktree Isolation Improvements
Enhanced git worktree management with better isolation guarantees for parallel agent execution.

## Changes

- Expanded swarm executor with additional orchestration capabilities
- Improved MCP server with new configuration options
- Updated A2A worker with refined message handling
- Enhanced CLI with new go/ralph integration commands

**Stats:** 17 files changed, 1,517 insertions(+), 117 deletions(-)
