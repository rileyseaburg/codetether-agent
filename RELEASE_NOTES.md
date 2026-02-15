# v3.0.2

## What's New

- **gRPC-Web Support**: Added gRPC-Web protocol with CORS enabled for browser-based clients, including a new voice streaming service (`src/a2a/voice_grpc.rs`)
- **MCP Bus Observer**: Live worktree monitoring with real-time event streaming through the MCP protocol (`src/mcp/bus_bridge.rs`, `src/tui/bus_log.rs`)
- **S3 Sink with Vault Integration**: Training data capture now supports HashiCorp Vault for credentials with configurable bucket and prefix
- **Ralph Enhancements**: Robust PRD JSON extraction, monorepo support, yield points for LLM request prioritization, and short-circuit quality gates
- **New Tools**:
  - `go` - Autonomous task execution pipeline (OKR → PRD → Ralph → results)
  - `swarm_execute` - Parallel sub-agent orchestration with configurable concurrency
  - `relay_autochat` - Agent-to-agent task delegation and result aggregation

## Bug Fixes

- Fixed worker endpoint migration from `/v1/opencode/` to `/v1/agent/` with normalized agent types
- Fixed S3 sink to add yield points, prioritizing LLM requests during heavy data uploads

## Changes

- Refactored S3 sink default bucket and prefix to `training` namespace
- Improved edit tools (`multiedit`, `confirm_edit`, `confirm_multiedit`, `advanced_edit`) with better error handling
- Enhanced TUI with expanded bus event visualization and navigation
- Added test commit script documentation to README

---

**Stats**: 49 files changed, 6,000+ insertions, 832 deletions
