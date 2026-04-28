# CodeTether Agent

[![Crates.io](https://img.shields.io/crates/v/codetether-agent.svg)](https://crates.io/crates/codetether-agent)
[![npm](https://img.shields.io/npm/v/codetether.svg)](https://www.npmjs.com/package/codetether)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/rileyseaburg/codetether-agent)](https://github.com/rileyseaburg/codetether-agent/releases)

A high-performance AI coding agent written in Rust. A2A (Agent-to-Agent) protocol support with dual JSON-RPC + gRPC transports, in-process agent message bus, rich terminal UI, parallel swarm execution, autonomous PRD-driven development, local FunctionGemma tool-call router, and **derived context per turn** — the canonical chat history stays append-only while the LLM sees a compressed, paired, and repaired ephemeral context every turn.

## Install

### Via npx (no Rust required)

```bash
npx codetether tui
npx codetether run "explain this codebase"
```

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh
```

Downloads the `codetether` binary. FunctionGemma is optional. No Rust toolchain required.

```bash
# Also install the FunctionGemma model (~292 MB)
curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh -s -- --functiongemma
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.ps1 | iex

# To also install FunctionGemma, run the script with -FunctionGemma
iwr https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.ps1 -OutFile install.ps1
.\install.ps1 -FunctionGemma
```

### From crates.io

```bash
cargo install codetether-agent

# Hardware acceleration for FunctionGemma:
cargo install codetether-agent --features candle-accelerate  # Apple Silicon / Intel Mac
cargo install codetether-agent --features candle-mkl         # Intel/AMD Linux (MKL)
cargo install codetether-agent --features candle-cuda        # NVIDIA GPU
```

### From Source

```bash
git clone https://github.com/rileyseaburg/codetether-agent
cd codetether-agent
cargo build --release
# Binary at target/release/codetether

# Without FunctionGemma (smaller binary)
cargo build --release --no-default-features
```

## Quick Start

### 1. Configure Provider Credentials

Provider API keys are loaded from HashiCorp Vault first. For local development,
CodeTether also detects common env vars and local AWS credentials unless
`CODETETHER_DISABLE_ENV_FALLBACK=1` is set.

```bash
export VAULT_ADDR="https://vault.example.com:8200"
export VAULT_TOKEN="hvs.your-token"

# Add a provider
vault kv put secret/codetether/providers/openrouter api_key="sk-or-v1-..."

# Production / hardened mode: require Vault-configured providers only.
export CODETETHER_DISABLE_ENV_FALLBACK=1
```

### 2. Launch the TUI

```bash
codetether tui
```

### 3. Or Run a Single Prompt

```bash
codetether run "explain this codebase"
```

## CLI

```bash
codetether tui                           # Interactive terminal UI
codetether run "prompt"                  # Single prompt
codetether run -- "/go <task>"           # Strategic relay (OKR-gated execution)
codetether run "/autochat <task>"        # Tactical relay (fast path)
codetether swarm "complex task"          # Parallel sub-agent execution
codetether swarm "complex task" --execution-mode k8s --k8s-pod-budget 4 --k8s-image <image>
codetether ralph run --prd prd.json      # Autonomous PRD-driven development
codetether ralph create-prd --feature X  # Generate a PRD template
codetether ralph status --prd prd.json   # Inspect PRD/story progress
codetether serve --port 4096             # HTTP server (A2A + cognition APIs)
codetether worker --server URL           # A2A worker mode
codetether spawn --name planner --peer http://localhost:4096/a2a  # Spawn A2A agent
codetether forage --loop --execute       # Autonomous OKR-governed work loop
codetether search "where is fn main"     # LLM-routed search across grep/web/RLM/memory
codetether oracle validate --query "find fn main" --file src/main.rs --payload-file result.json
codetether mcp list-tools                # List built-in MCP-exposed tools
codetether auth codex                    # OAuth login for OpenAI Codex
codetether auth copilot --client-id ID   # OAuth login for GitHub Copilot
codetether index --path src --json       # Build codebase index (local embeddings)
codetether moltbook profile              # Inspect your Moltbook profile
codetether okr list                      # List OKRs
codetether okr create --title "Ship feature X" --description "Customer-visible milestone" --target 100
codetether okr report --id <uuid>        # Show OKR or run report
codetether pr create --title "feat: add X"  # Create/update pull requests
codetether models                        # List available models from all providers
codetether stats                         # Telemetry & execution statistics
codetether benchmark                     # Run model benchmark suite
codetether cleanup                       # Clean orphaned worktrees
codetether config --show                 # Show config
codetether context reset                 # Emit a [CONTEXT RESET] marker in the session
codetether context browse list           # List virtual session history paths
codetether context browse show-turn N    # Show turn N as markdown
```

`codetether index` always generates embeddings locally (no paid API required). Tune with `--embedding-model`, `--embedding-dimensions`, `--embedding-batch-size`, and `--embedding-input-chars`.

### Additional Command Families

```bash
# Auth workflows
codetether auth login --server https://api.codetether.run
codetether auth register --server https://api.codetether.run
codetether auth cookies --provider gemini-web --file cookies.txt

# MCP client mode
codetether mcp connect npx -y @modelcontextprotocol/server-filesystem .
codetether mcp call --tool read --arguments '{"path":"README.md"}'

# Oracle maintenance
codetether oracle sync --json

# Moltbook
codetether moltbook register my-agent --description "Autonomous coding agent"
codetether moltbook status
codetether moltbook profile
```

### Forage: Autonomous OKR-Governed Loop

```bash
# Scan for top opportunities
codetether forage --top 5
codetether forage --top 5 --no-s3            # Local only, skip S3

# Autonomous loop
codetether forage --loop --interval-secs 120 --top 3

# Autonomous + execute
codetether forage --loop --execute --interval-secs 120 --top 3

# Smart swarms in forage loop
codetether forage --loop --execute --execution-engine swarm --interval-secs 120 --top 3 \
  --swarm-max-subagents 8 --swarm-strategy auto --model openai-codex/gpt-5.5

# Moonshot rubric: mission statements that bias prioritization
codetether forage --loop --execute --execution-engine swarm --interval-secs 120 --top 3 \
  --moonshot "Autonomous agents continuously ship measurable customer value" \
  --moonshot "Reliability first: no data loss in long-running autonomy"

# Strict moonshot gate
codetether forage --loop --execute --execution-engine swarm --interval-secs 120 --top 3 \
  --moonshot-file ./.codetether-agent/moonshots.txt \
  --moonshot-required --moonshot-min-alignment 0.25
```

Notes:
- `--execute` mode auto-seeds a default mission OKR if the repository is empty so the loop can self-start.
- Without `--execute`, forage only reports existing opportunities.
- KR progress is only recorded when quality gates (cargo check, cargo test) pass.

## Security

CodeTether treats security as non-optional infrastructure, not a feature flag.

| Control | Implementation |
|---------|----------------|
| **Authentication** | Mandatory Bearer token on every endpoint (except `/health`). Cannot be disabled. |
| **Audit Trail** | Append-only JSON Lines log of every action — queryable by actor, action, resource, time range. |
| **Plugin Signing** | Ed25519 signatures on tool manifests. SHA-256 content hashing. Unsigned tools rejected. |
| **Sandboxing** | Resource-limited execution: max memory, max CPU seconds, network allow/deny per tool. |
| **Secrets** | All API keys stored in HashiCorp Vault — never in config files or environment variables. |
| **K8s Self-Healing** | Reconciliation loop detects unhealthy pods and triggers rolling restarts. |

## Features

### Derived Context: Append-Only History + Ephemeral LLM Context

Every turn the agent **derives** a fresh `DerivedContext` from the canonical `session.messages` — the transcript stays append-only and the LLM gets a compressed, paired, and repaired view. This separation means:

- **True history** — `session.messages` is never rewritten by compression, so `/undo`, `/fork`, and session recall see every original turn.
- **Compression safety** — Context-window enforcement runs on a clone, not the source of truth.
- **Tool-call pairing repair** — Orphaned tool calls get synthetic placeholders so the provider never sees dangling `assistant.tool_calls` without matching `tool` results.
- **Policy-driven resets** — Lu et al. reset-to-(prompt, summary) when estimated tokens exceed a threshold, via the `DerivePolicy::Reset` policy.
- **Mid-session recall** — The `session_recall` tool recovers details from the canonical history that the compressor may have dropped from the derived context.

The derivation pipeline: clone → compress last oversized message → experimental pairing → adaptive budget cascade → orphan repair → `DerivedContext { messages, compressed, origin_len }`.

```bash
# Environment variables for history persistence (optional)
export CODETETHER_HISTORY_S3_ENDPOINT="http://localhost:9000"
export CODETETHER_HISTORY_S3_BUCKET="codetether-history"
export CODETETHER_HISTORY_S3_ACCESS_KEY="minioadmin"
export CODETETHER_HISTORY_S3_SECRET_KEY="minioadmin"
```

### FunctionGemma Tool Router

Your primary LLM (Claude, GPT-4o, Kimi, etc.) focuses on reasoning. A local model ([FunctionGemma](https://huggingface.co/google/functiongemma-270m-it), 270M params) handles structured tool-call formatting via Candle inference (~5-50ms on CPU).

- **Provider-agnostic** — Switch models freely; tool-call behavior stays consistent.
- **Zero overhead** — If the LLM already returns tool calls, FunctionGemma is never invoked.
- **Safe degradation** — On any error, the original response is returned unchanged.

```bash
export CODETETHER_TOOL_ROUTER_ENABLED=true
export CODETETHER_TOOL_ROUTER_MODEL_PATH="$HOME/.local/share/codetether/models/functiongemma/functiongemma-270m-it-Q8_0.gguf"
export CODETETHER_TOOL_ROUTER_TOKENIZER_PATH="$HOME/.local/share/codetether/models/functiongemma/tokenizer.json"
```

| Variable | Default | Description |
|----------|---------|-------------|
| `CODETETHER_TOOL_ROUTER_ENABLED` | `false` | Activate the router |
| `CODETETHER_TOOL_ROUTER_MODEL_PATH` | — | Path to `.gguf` model |
| `CODETETHER_TOOL_ROUTER_TOKENIZER_PATH` | — | Path to `tokenizer.json` |
| `CODETETHER_TOOL_ROUTER_ARCH` | `gemma3` | Architecture hint |
| `CODETETHER_TOOL_ROUTER_DEVICE` | `auto` | `auto` / `cpu` / `cuda` |
| `CODETETHER_TOOL_ROUTER_MAX_TOKENS` | `512` | Max decode tokens |
| `CODETETHER_TOOL_ROUTER_TEMPERATURE` | `0.1` | Sampling temperature |

### RLM: Recursive Language Model

Handles content that exceeds model context windows. Loads context into a REPL, lets the LLM explore it with structured tool calls (`rlm_head`, `rlm_tail`, `rlm_grep`, `rlm_count`, `rlm_slice`, `rlm_llm_query`), and returns a synthesized answer via `rlm_final`.

```bash
codetether rlm "What are the main functions?" -f src/large_file.rs
cat logs/*.log | codetether rlm "Summarize the errors" --content -
```

#### Local CUDA

```bash
cargo install --path . --force --features candle-cuda,functiongemma

export LOCAL_CUDA_MODEL="qwen3.5-9b"
export LOCAL_CUDA_MODEL_PATH="$HOME/models/qwen3-4b/Qwen3-4B-Q4_K_M.gguf"
export LOCAL_CUDA_TOKENIZER_PATH="$HOME/models/qwen3-4b/tokenizer.json"

codetether rlm --model local_cuda/qwen3.5-9b --file src/rlm/repl.rs --json \
  "Find all occurrences of 'async fn' in src/rlm/repl.rs"
```

#### Content Types

| Type | Detection | Optimization |
|------|-----------|--------------|
| `code` | Function definitions, imports | Semantic chunking by symbols |
| `logs` | Timestamps, log levels | Time-based chunking |
| `conversation` | Chat markers, turns | Turn-based chunking |
| `documents` | Markdown headers, paragraphs | Section-based chunking |

### OKR-Driven Execution

CodeTether uses **OKRs (Objectives and Key Results)** as the bridge between business strategy and autonomous agent execution. Instead of handing agents a task and hoping for the best, you state your intent, approve a plan, and get measurable outcomes.

#### The `/go` Lifecycle

```
┌──────────────────────────────────────────────────────────────────┐
│                        /go Lifecycle                             │
│                                                                  │
│  1. You state intent                                             │
│     └─ "/go audit the bin cleaning system for Q3 readiness"      │
│                                                                  │
│  2. System reframes as OKR                                       │
│     └─ Objective + Key Results generated from your prompt        │
│                                                                  │
│  3. You approve or deny                                          │
│     └─ TUI: press A (approve) or D (deny)                        │
│     └─ CLI: y/n prompt                                           │
│                                                                  │
│  4. Autonomous relay execution                                   │
│     └─ Swarms, tools, sequential agent turns                     │
│                                                                  │
│  5. KR progress updates (per relay turn)                         │
│     └─ Key Results evaluated and persisted after each turn       │
│                                                                  │
│  6. Completion + outcome                                         │
│     └─ Final KR outcomes recorded                                │
└──────────────────────────────────────────────────────────────────┘
```

#### `/go` vs `/autochat`

| Command | Purpose | OKR Gate | Best For |
|---------|---------|----------|----------|
| `/go` | Strategic execution | Yes — draft → approve → run | Epics, business goals, tracked outcomes |
| `/autochat` | Tactical execution | No — runs immediately | Quick tasks, bug fixes |

OKRs naturally support long-running work with persistent state, cumulative KR progress, checkpointed relays for crash recovery, and correlation IDs (`okr_id`, `okr_run_id`, `relay_id`, `session_id`) across all audit/event entries.

```bash
codetether okr list                     # List all OKRs
codetether okr create --title "Reduce p95 latency" --description "Execution latency initiative" --target 100
codetether okr status --id <uuid>       # Detailed status
codetether okr runs --id <uuid>         # List runs
codetether okr report --id <uuid>       # Full report
codetether okr export --id <uuid>       # Export as JSON
codetether okr stats                    # Aggregate stats
```

### Session Management

The TUI provides first-class session lifecycle commands:

| Command | Purpose |
|---------|---------|
| `/ask <question>` | Ask a one-off question without adding to session history |
| `/undo [N]` | Remove last `N` user/assistant/tool turns from the session |
| `/fork [N]` | Create a child session from the current state (optionally at turn `N`) |
| `/audit` | Open the audit view to inspect action history |

Sessions remain **append-only** — `/undo` and `/fork` operate on the canonical transcript, while the LLM always receives a freshly derived ephemeral context per turn.

### Swarm: Parallel Sub-Agent Execution

Decomposes complex tasks into subtasks and executes them concurrently.

```bash
codetether swarm "Implement user auth with tests and docs"
codetether swarm "Refactor the API layer" --strategy domain --max-subagents 8
codetether swarm "Ship feature X" --execution-mode k8s --k8s-pod-budget 6 --k8s-image <image>
```

Strategies: `auto` (default), `domain`, `data`, `stage`, `none`.

Execution modes:
- `local` (default): sub-agents run as local async tasks.
- `k8s`: sub-agents run as isolated Kubernetes pods with deterministic collapse-based pruning/promotion.

### Ralph: Autonomous PRD-Driven Development

Give it a spec, watch it work story by story. Each iteration is a fresh agent with full tool access. Memory persists via git history, `progress.txt`, and the PRD file.

```bash
codetether ralph create-prd --feature "User Auth" --project-name my-app
codetether ralph run --prd prd.json --max-iterations 10
codetether ralph status --prd prd.json
```

Terminal outcomes: `Completed` (all stories passed), `MaxIterations` (partial), `QualityFailed` (no stories passed gates).

### Oracle: Deterministic Validation Utilities

Validate structured answers against source material and sync oracle traces to remote storage.

```bash
codetether oracle validate --query "find fn main" --file src/main.rs --payload-file result.json
codetether oracle sync --json
```

### Moltbook

Moltbook is the social network integration for agents. The CLI supports registration, claim status, profile management, posting, introductions, heartbeat/feed checks, comments, and search.

```bash
codetether moltbook register my-agent --description "Autonomous coding agent"
codetether moltbook status
codetether moltbook profile
codetether moltbook post "Hello" --content "Shipping updates" --submolt general
codetether moltbook search "codetether"
```

### TUI

Rich terminal UI with model selector, session picker, swarm view, Ralph view, audit view, and theme hot-reload.

**Slash Commands**: `/go`, `/autochat`, `/ask`, `/new`, `/model`, `/sessions`, `/swarm`, `/ralph`, `/rlm`, `/bus`, `/lsp`, `/latency`, `/symbols`, `/settings`, `/file`, `/image`, `/spawn`, `/kill`, `/agents`, `/undo`, `/fork`, `/audit`, `/autoapply`, `/network`, `/mcp connect|servers|tools|call`, `/import-codex`, `/keys`, `/help`

**Keyboard**: `Ctrl+M` model selector, `Ctrl+B` toggle layout, `Ctrl+S`/`F2` swarm view, `Tab` switch agents, `Alt+j/k` scroll, `?` help

## Providers

| Provider | Default Model | Notes |
|----------|---------------|-------|
| `zai` | `glm-5` | Z.AI flagship — GLM-5 agentic coding (200K ctx) |
| `moonshotai` | `kimi-k2.5` | Excellent for coding |
| `github-copilot` | `claude-opus-4` | GitHub Copilot models |
| `openai` | `gpt-4o` | OpenAI GPT models |
| `openai-codex` | `gpt-5.5` | ChatGPT subscription OAuth |
| `openrouter` | `stepfun/step-3.5-flash:free` | Access to many models |
| `google` | `gemini-2.5-pro` | Google AI |
| `anthropic` | `claude-sonnet-4-20250514` | Direct API |
| `stepfun` | `step-3.5-flash` | Chinese reasoning model |
| `vertex-glm` | `zai-org/glm-5-maas` | GLM-5 via Vertex AI (service account JWT) |
| `vertex-anthropic` | `claude-sonnet-4-20250514` | Claude via GCP Vertex AI |
| `bedrock` | `amazon.nova-lite-v1:0` / `us.anthropic.claude-opus-4-6-v1:0` | Amazon Bedrock Converse API |
| `local-cuda` | (configurable) | Local CUDA inference via Candle (Qwen, etc.) |
| `gemini-web` | `gemini-2.5-pro` | Google Gemini web-based (cookie auth) |

All keys stored in Vault at `secret/codetether/providers/<name>`.

## Tools

50+ built-in tools include file ops (`read`, `write`, `edit`, `multiedit`, `apply_patch`, `glob`, `list`, `tree`, `fileinfo`, `headtail`, `diff`), code intelligence (`lsp`, `grep`, `codesearch`, `advanced_edit`), execution (`bash`, `batch`, `task`), browser automation (`browserctl`), web (`webfetch`, `websearch`), media (`image`, `voice`, `podcast`, `youtube`, `avatar`), planning (`ralph`, `prd`, `okr`, `todoread`, `todowrite`, `plan_enter`, `plan_exit`), session and safety (`context_reset`, `context_browse`, `session_recall`, `session_task`, `undo`, `question`, `confirm_edit`, `confirm_multiedit`), agent orchestration (`agent`, `swarm_execute`, `swarm_share`, `relay_autochat`, `go`, `rlm`), knowledge (`memory`, `skill`, `mcp`), and infrastructure (`kubernetes`). Provider-backed agent registries also expose the LLM-routed `search` tool. Compatibility aliases `patch`, `file_info`, `head_tail`, `todo_read`, `todo_write`, `mcp_bridge`, and `k8s_tool` remain accepted.

## MCP Server

CodeTether exposes 30+ tools via the [Model Context Protocol](https://modelcontextprotocol.io/) over stdio. This lets AI clients (GitHub Copilot in VS Code, Claude Desktop, etc.) call CodeTether tools directly.

### VS Code (Workspace-Level)

Add `.vscode/mcp.json` to your workspace:

```json
{
  "servers": {
    "codetether": {
      "command": "/home/riley/.cargo/bin/codetether",
      "args": ["mcp", "serve"],
      "env": {
        "RUST_LOG": "error"
      }
    }
  }
}
```

### Claude Desktop

Edit `~/.config/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "codetether": {
      "command": "/path/to/codetether",
      "args": ["mcp", "serve"],
      "env": { "RUST_LOG": "error" }
    }
  }
}
```

For remote machines over SSH:

```json
{
  "mcpServers": {
    "codetether": {
      "command": "ssh",
      "args": ["-T", "user@host", "cd /project && RUST_LOG=error /path/to/codetether mcp serve"]
    }
  }
}
```

### Codex CLI

Add to `~/.codex/config.toml`:

```toml
[mcp_servers.codetether]
command = "/absolute/path/to/codetether"
args = ["mcp", "serve", "/absolute/workspace/path"]
```

### Exposed Tools (30+)

| Category | Tools |
|----------|-------|
| **File Ops** | `read`, `write`, `edit`, `multiedit`, `apply_patch`, `glob`, `list`, `tree`, `fileinfo`, `headtail`, `diff` |
| **Search** | `grep`, `codesearch`, `advanced_edit` |
| **Execution** | `bash`, `batch`, `task` |
| **Browser** | `browserctl` |
| **Code Intelligence** | `lsp` (includes diagnostics from eslint, ruff, biome, stylelint) |
| **Web** | `webfetch`, `websearch` |
| **Media** | `image`, `voice`, `podcast`, `youtube`, `avatar` |
| **Session & Safety** | `context_reset`, `context_browse`, `session_recall`, `session_task`, `undo`, `question`, `confirm_edit`, `confirm_multiedit` |
| **Agent Orchestration** | `agent`, `swarm_execute`, `swarm_share`, `relay_autochat`, `go`, `rlm` |
| **Planning** | `ralph`, `prd`, `okr`, `todoread`, `todowrite` |
| **Knowledge** | `memory`, `skill`, `mcp` (bridge to other MCP servers) |
| **Infrastructure** | `kubernetes` |

The MCP registry also accepts compatibility aliases `patch`, `file_info`, `head_tail`, `todo_read`, `todo_write`, `mcp_bridge`, and `k8s_tool`.

```bash
codetether mcp list-tools          # List available MCP tools
codetether mcp list-tools --json   # JSON output
codetether mcp serve               # Start stdio MCP server
codetether mcp serve --bus-url URL # With agent bus integration
codetether mcp connect npx -y @modelcontextprotocol/server-filesystem .
codetether mcp call --tool read --arguments '{"path":"README.md"}'
```

## A2A Protocol

Dual-transport Agent-to-Agent communication with a shared in-process bus:

- **Worker mode** — Connect to the CodeTether platform and process tasks.
- **Server mode** — Accept tasks via JSON-RPC (Axum, `:4096`) and gRPC (Tonic, `:50051`) simultaneously.
- **Spawn mode** — Launch a standalone A2A peer that auto-registers and discovers other peers. See [`docs/a2a-spawn.md`](docs/a2a-spawn.md) for the full two-terminal / multi-repo walkthrough, JSON-RPC reference, and discovery internals.
- **Bus mode** — In-process pub/sub for zero-latency local agent communication.

### Transports

| Transport | Port | Use Case |
|-----------|------|----------|
| JSON-RPC (Axum) | `4096` | REST API, SSE streams, `/.well-known/agent.json` |
| gRPC (Tonic) | `50051` | High-frequency A2A RPCs, streaming |
| In-Process Bus | — | Local sub-agents, swarm coordination |

### gRPC RPCs

| RPC | Description |
|-----|-------------|
| `SendMessage` | Submit a task/message |
| `SendStreamingMessage` | Submit with streaming status updates |
| `GetTask` | Retrieve task by ID |
| `CancelTask` | Cancel a running task |
| `TaskSubscription` | Subscribe to status updates (server-stream) |
| `CreateTaskPushNotificationConfig` | Register push notification endpoint |
| `GetTaskPushNotificationConfig` | Get push notification config |
| `ListTaskPushNotificationConfig` | List push configs for a task |
| `DeleteTaskPushNotificationConfig` | Remove a push notification config |
| `GetAgentCard` | Retrieve the agent's capability card |

### Agent Bus Topics

| Topic Pattern | Semantics |
|---------------|-----------|
| `agent.{id}` | Messages to a specific agent |
| `task.{id}` | All updates for a task |
| `swarm.{id}` | Swarm-level coordination |
| `broadcast` | Global announcements |
| `results.{key}` | Shared result publication |
| `tools.{name}` | Tool-specific channels |

### Cognition APIs

When running `codetether serve`, perpetual persona swarms with SSE event stream:

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/cognition/start` | Start perpetual cognition loop |
| `POST` | `/v1/cognition/stop` | Stop cognition loop |
| `GET` | `/v1/cognition/status` | Runtime status and metrics |
| `GET` | `/v1/cognition/stream` | SSE stream of thought events |
| `POST` | `/v1/swarm/personas` | Create a root persona |
| `POST` | `/v1/swarm/personas/{id}/spawn` | Spawn child persona |
| `POST` | `/v1/swarm/personas/{id}/reap` | Reap a persona |
| `GET` | `/v1/swarm/lineage` | Persona lineage graph |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    CodeTether Platform                      │
│                 (A2A Server at api.codetether.run)          │
└───────────────┬───────────────────────┬─────────────────────┘
                │ SSE/JSON-RPC          │ gRPC (A2A proto)
                ▼                       ▼
┌─────────────────────────────────────────────────────────────┐
│                    codetether-agent                         │
│                                                             │
│   ┌───────────────────────────────────────────────────┐     │
│   │              Agent Message Bus                    │     │
│   │   (broadcast pub/sub, topic routing, BusHandle)   │     │
│   └──┬──────────┬──────────┬──────────┬───────────────┘     │
│      │          │          │          │                      │
   ┌──┴───┐  ┌──┴───┐  ┌──┴───┐  ┌──┴────────┐  ┌────────┐ │
   │ A2A  │  │ Swarm│  │ Tool │  │  Provider │  │Derived │ │
   │Worker│  │ Exec │  │System│  │   Layer   │  │Context │ │
   └──┬───┘  └──┬───┘  └──┬───┘  └──┬────────┘  └────────┘ │
│      │         │         │         │                        │
│   ┌──┴─────────┴─────────┴─────────┴──┐                     │
│   │         Agent Registry            │                     │
│   └───────────────────────────────────┘                     │
│                                                             │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐ ┌──────────┐    │
│   │JSON-RPC  │  │ gRPC     │  │ Auth     │ │ Audit    │    │
│   │(Axum)    │  │ (Tonic)  │  │ (Bearer) │ │ (JSONL)  │    │
│   │:4096     │  │ :50051   │  │ Mandatory│ │ Append   │    │
│   └──────────┘  └──────────┘  └──────────┘ └──────────┘    │
│   ┌──────────┐  ┌──────────┐  ┌──────────────────────┐     │
│   │ Sandbox  │  │ K8s Mgr  │  │  HashiCorp Vault     │     │
│   │ (Ed25519)│  │ (Deploy) │  │  (API Keys)          │     │
│   └──────────┘  └──────────┘  └──────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

## Configuration

`~/.config/codetether-agent/config.toml`:

```toml
[default]
provider = "anthropic"
model = "claude-sonnet-4-20250514"

[ui]
theme = "marketing"   # marketing, dark, light, solarized-dark, solarized-light

[session]
auto_save = true

[lsp]
[lsp.servers]
# my-ruby-lsp = { command = "ruby-lsp", args = ["--stdio"], file_extensions = ["rb"] }

[lsp.linters]
eslint = { enabled = true }
ruff = { enabled = true }
biome = { enabled = false }
stylelint = { enabled = true }
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VAULT_ADDR` | — | Vault server address |
| `VAULT_TOKEN` | — | Authentication token |
| `VAULT_MOUNT` | `secret` | KV mount path |
| `VAULT_SECRETS_PATH` | `codetether/providers` | Provider secrets prefix |
| `CODETETHER_AUTH_TOKEN` | (auto-generated) | Bearer token for API auth |
| `CODETETHER_DATA_DIR` | `.codetether-agent` | Runtime data directory |
| `CODETETHER_GRPC_PORT` | `50051` | gRPC server port |
| `CODETETHER_A2A_PEERS` | — | Comma-separated peer seed URLs |

## Performance

| Metric | Value |
|--------|-------|
| Startup | 13ms |
| Memory (idle) | ~15 MB |
| Memory (10-agent swarm) | ~55 MB |
| Binary size | ~12.5 MB |

Written in Rust with tokio — true parallelism, no GC pauses, native performance. See [CHANGELOG.md](CHANGELOG.md) for benchmark details.

## Development

```bash
cargo build                  # Debug build
cargo build --release        # Release build
cargo test                   # Run tests
cargo clippy --all-features  # Lint
cargo fmt                    # Format
```

## License

MIT
