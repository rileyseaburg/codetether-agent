# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **CloudEvent Task Notification**: Worker now receives task notifications via Knative Eventing
  - `/task` endpoint extracts `task_id` from CloudEvent payload
  - Worker loop immediately polls for pending tasks when notified
  - Enables real-time task dispatch without SSE polling delay

### Deprecated

- **opencode API endpoints**: The `/v1/opencode/*` endpoints are deprecated
  - Use `/v1/tasks/dispatch` for task creation with Knative integration
  - Use `/v1/worker/tasks/*` endpoints for worker operations
  - Migration guide will be provided before removal in v3.0.0

### Fixed

- **Windows Installer** (`install.ps1`):
  - Now tries multiple artifact formats to support both GitHub Actions releases (msvc + zip) and Jenkins releases (gnu + tar.gz)
  - Automatically detects and downloads the correct binary format
  - Improved error messages when no binary is available

## [1.1.0] - 2026-02-10

### Added

- **Mandatory Authentication Middleware** (`src/server/auth.rs`):
  - Bearer token auth layer that cannot be disabled
  - Auto-generates HMAC-SHA256 token from hostname + timestamp if `CODETETHER_AUTH_TOKEN` not set
  - Exempts only `/health` endpoint
  - Returns structured JSON 401 errors for unauthorized requests

- **System-Wide Audit Trail** (`src/audit/mod.rs`):
  - Every API call, tool execution, and session event is logged
  - Append-only JSON Lines file backend
  - Queryable with filters: actor, action, resource, time range
  - Global singleton via `AUDIT_LOG` — initialized once at server startup
  - New endpoints: `GET /v1/audit/events`, `POST /v1/audit/query`

- **Plugin Sandboxing & Code Signing** (`src/tool/sandbox.rs`):
  - Tool manifest system with SHA-256 content hashing
  - Ed25519 signature verification for all plugin manifests
  - Sandbox policies: Default, Restricted, Custom
  - Resource limits: max memory, max CPU seconds, network allow/deny
  - `ManifestStore` registry for signed tool manifests

- **Kubernetes Self-Deployment** (`src/k8s/mod.rs`):
  - Auto-detects cluster via `KUBERNETES_SERVICE_HOST`
  - Self-info from Kubernetes Downward API
  - `ensure_deployment()` creates or updates Deployments
  - `scale()` adjusts replica count
  - `health_check()` with rolling restart of unhealthy pods
  - `reconcile_loop()` background task running every 30 seconds
  - New endpoints: `GET /v1/k8s/status`, `POST /v1/k8s/scale`, `POST /v1/k8s/health`, `POST /v1/k8s/reconcile`

### Dependencies

- Added `kube` 0.98 (runtime, client, derive)
- Added `k8s-openapi` 0.22 (v1_31)
- Added `sha2` 0.10
- Added `ed25519-dalek` 2
- Added `base64` 0.22

### Fixed

- Pre-existing lifetime bug in `a2a/worker.rs` (`catalog_alias` closure)

## [0.1.5] - 2026-02-07

### Added

- **Perpetual Persona Swarms (Phase 0)**: Initial always-on cognition runtime in `codetether-agent`
  - New cognition contracts for persona identity/policy/state, thought events, proposals, snapshots, and lineage graph
  - In-memory perpetual loop with bounded buffers and guarded recursion limits (spawn depth + branching factor)
  - New server APIs for cognition control and swarm lifecycle:
    - `POST /v1/cognition/start`
    - `POST /v1/cognition/stop`
    - `GET /v1/cognition/status`
    - `GET /v1/cognition/stream` (SSE)
    - `GET /v1/cognition/snapshots/latest`
    - `POST /v1/swarm/personas`
    - `POST /v1/swarm/personas/{id}/spawn`
    - `POST /v1/swarm/personas/{id}/reap`
    - `GET /v1/swarm/lineage`
  - Feature flags for runtime enablement, auto-start, loop interval, and cognition budgets
  - Documentation added in `docs/perpetual_persona_swarms.md`
- **Amazon Bedrock Provider**:
  - New provider module using Bedrock Converse API with bearer-token auth
  - Registered provider aliases: `bedrock` and `aws-bedrock`
  - Supports configurable AWS region from Vault provider secret metadata (`region`)
- **OpenAI-Compatible Model Catalog Defaults**:
  - Added known-model fallback listings for `cerebras`, `novita`, and `minimax`

### Changed

- **A2A Worker Server Paths**:
  - Updated worker registration, task polling/output, and heartbeat endpoints to `/v1/opencode/*`

### Fixed

- **Model Reference Translation**:
  - Prevented `provider:model` conversion when model IDs already contain `/`
  - Avoids corrupting model IDs that use `:` for versions (for example `amazon.nova-micro-v1:0`)

## [0.1.2] - 2026-02-05

### Added

- **TUI Swarm Mode UI**: Real-time visualization of parallel sub-agent execution
  - `/swarm <task>` command to run tasks in parallel swarm mode
  - Live progress display with subtask status, stages, and execution stats
  - Toggle views with `Ctrl+S` or `/view` command

- **Session Management**: Resume conversations across TUI sessions
  - `/sessions` - List recent saved sessions
  - `/resume` - Load most recent session
  - `/resume <id>` - Load specific session by ID
  - `/new` - Start fresh session (auto-saves current)

- **Real-time Tool Streaming**: See tool calls as they execute
  - Live status indicator shows current tool being run
  - Tool calls and results displayed with distinct styling

- **TUI Enhancements**:
  - Auto-scroll to bottom on message send/receive
  - Theme configuration support
  - Token usage display with cost tracking
  - File edit confirmation prompts
  - Keyboard navigation improvements

### Fixed

- **Token Cost Calculation**: Was ~1 trillion times too high, now correctly calculates costs
- **Vim Key Conflicts**: Added Alt/Ctrl modifiers so h,j,k,l,g,d,u can be typed normally
- **Swarm UI Updates**: Subtasks now show immediately after decomposition, not just at completion
- **F2 Key Binding**: Added `Ctrl+S` as reliable alternative (F2 doesn't work in many terminals)

### Changed

- **Default Model**: TUI now defaults to `zhipuai/glm-4.7` for better performance
- **Help Menu**: Condensed and added slash command documentation

## [0.2.0] - 2026-02-03

### Added

- **Library Support**: Added `lib.rs` for using codetether-agent as a library
- **Integration Tests**: Comprehensive A2A client integration tests with mock server
- **Design Documentation**: Architecture docs for agent-swarm integration, MCP metadata, session management
- **SwarmMetrics**: Execution metrics tracking for swarm operations
- **MCP Registry**: Tool discovery and registration via McpRegistry

### Changed

- **async-openai 0.32.4**: Updated to latest API with new chat completion types
- **A2A Client**: Made public with exposed `call_rpc()` for custom RPC calls
- **Tool Registry**: Now includes BatchTool and InvalidTool by default
- **Provider Logging**: Enhanced debug logging with API key validation

### Fixed

- **67 Compiler Warnings → 0**: All unused code now properly integrated via swarm dogfooding
  - MCP server tool/resource metadata storage
  - Session add_message and generate_title methods
  - Provider API key usage in debug output
  - Error constants (PARSE_ERROR, INVALID_REQUEST, etc.)
  - Context summary logging in RLM repl

### Dogfooding

This release was refactored by the swarm system itself:
- **Model**: Kimi K2.5 via Moonshot
- **Duration**: ~25 minutes
- **Files Changed**: 54
- **Lines Added**: 5,507
- **Warnings Fixed**: 67 → 0

## [0.1.0] - 2026-02-03

### Added

#### Core Features
- **Ralph Loop**: Autonomous PRD-driven development loop
  - Implements user stories from structured PRD files
  - Quality gates (cargo check, clippy, test, build) per story
  - Memory persistence via git history, progress.txt, and PRD updates
  - CLI: `codetether ralph run -p prd.json -m "model" --max-iterations N`

- **Swarm Execution**: Parallel sub-agent execution for complex tasks
  - Automatic task decomposition with multiple strategies (auto, domain, stage)
  - Concurrent sub-agent execution with result aggregation
  - CLI: `codetether swarm "task description" --strategy auto`

- **RLM (Recursive Language Model)**: Handle large contexts exceeding model windows
  - REPL-like environment for context exploration (head, tail, grep, count)
  - Sub-LM calls via `llm_query()` for semantic sub-questions
  - Auto-detection of content types (code, logs, conversation, documents)

#### Providers
- **Moonshot Direct**: Native Moonshot AI integration (kimi-k2.5)
- **OpenRouter**: Access to 100+ models via OpenRouter API
- **StepFun**: Step 3.5 Flash support
- **OpenAI**: GPT-4, GPT-4o, o1, o3 models
- **Anthropic**: Claude 3.5 Sonnet, Claude 4 Opus
- **Google**: Gemini Pro, Gemini Flash
- **DeepSeek**: DeepSeek V3

#### Tools (24+ total)
- `ralph` - Autonomous PRD-driven agent loop
- `rlm` - Recursive Language Model for large contexts
- `prd` - Generate and manage PRD documents
- `lsp` - Language Server Protocol operations (definition, references, hover, completion)
- `batch` - Run multiple tool calls in parallel
- `task` - Background task execution
- `codesearch` - Semantic code search
- `websearch` - Web search integration
- `webfetch` - Fetch web pages with smart extraction
- `skill` - Execute learned skills
- `todo_read`/`todo_write` - Track task progress
- `question` - Ask clarifying questions
- `plan_enter`/`plan_exit` - Switch to planning mode
- `read_file`/`write_file` - File operations
- `edit`/`multiedit` - Search/replace edits
- `apply_patch` - Apply unified diff patches
- `grep` - Search file contents with regex
- `bash` - Execute shell commands
- `glob` - Find files by pattern
- `list_dir` - List directory contents

#### Infrastructure
- **A2A Protocol**: Worker, server, and client modes for agent-to-agent communication
- **MCP Support**: Model Context Protocol client/server
- **HashiCorp Vault**: Secure API key storage (no env var secrets)
- **Interactive TUI**: Terminal interface built with Ratatui
- **Session Management**: Persistent session history with git-aware storage

### Dogfooding Achievement

This release was **built using itself**. The Ralph loop autonomously implemented 20 user stories:

| Metric | Value |
|--------|-------|
| Total Stories | 20 |
| Pass Rate | 100% |
| Time | 29.5 minutes |
| Cost | $3.75 |
| Model | Kimi K2.5 |

Stories implemented:
- LSP Client (10 stories): Transport, JSON-RPC, Initialize, didOpen, didChange, Completion, Hover, Definition, Shutdown, Configuration
- Missing Features (10 stories): External Directory, RLM Pool, Truncation, LSP Integration, References, and more

### Performance

| Metric | CodeTether (Rust) | opencode (Bun) | Advantage |
|--------|-------------------|----------------|-----------|
| Binary Size | 12.5 MB | ~90 MB | 7.2x smaller |
| Startup Time | 13 ms | 25-50 ms | 2-4x faster |
| Memory (10 agents) | 55 MB | 280 MB | 5.1x less |
| Process Spawn | 1.5 ms | 5-10 ms | 3-7x faster |
