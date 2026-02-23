# CodeTether Agent

[![Crates.io](https://img.shields.io/crates/v/codetether-agent.svg)](https://crates.io/crates/codetether-agent)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/rileyseaburg/codetether-agent)](https://github.com/rileyseaburg/codetether-agent/releases)

A high-performance AI coding agent written in Rust. First-class A2A (Agent-to-Agent) protocol support with dual JSON-RPC + gRPC transports, in-process agent message bus, rich terminal UI, parallel swarm execution, autonomous PRD-driven development, and a local FunctionGemma tool-call router that separates reasoning from formatting.

## v4.0.0 — Major Release: Hybrid Swarm, Local CUDA & Real-Time Task Dispatch

CodeTether Agent `v4.0.0` is a major release delivering zero-latency local inference, Kubernetes-native real-time task dispatch, and a significantly reduced cloud cost model.

- **Hybrid Swarm Architecture & Local CUDA Provider** — New `LocalCudaProvider` runs ML inference directly on NVIDIA hardware via Candle, achieving zero network latency. Strategic reasoning stays on cloud models (e.g., Claude Opus); tool-call formatting is offloaded to local FunctionGemma. Full architecture documented in `docs/architecture/hybrid_swarm.md`.
- **CloudEvent Task Notification** — Worker now receives task notifications via Knative Eventing. The `/task` endpoint extracts `task_id` from the CloudEvent payload and immediately polls for pending tasks, eliminating SSE polling delay.
- **Claude Opus 4.6 Bedrock Pricing** — Reflects new Amazon Bedrock rates: input $5.00/1M tokens (was $15.00, −67%) and output $25.00/1M tokens (was $75.00, −67%). 200 K context limit added for Opus 4.6. TUI token display updated.
- **Windows Installer Fix** — `install.ps1` now tries multiple artifact formats (msvc + zip for GitHub Actions, gnu + tar.gz for Jenkins) and auto-detects the correct binary. Improved error messages when no binary is available.

### Upgrading to v4.0

- Existing configurations, Vault secrets, and sessions are forward-compatible with v4.
- Worker deployments automatically gain CloudEvent push-based dispatch; no `--server` polling changes required.
- Enable local CUDA inference by building with `--features candle-cuda` and setting `CODETETHER_TOOL_ROUTER_DEVICE=cuda`.
- Amazon Bedrock users: update cost-tracking dashboards to reflect the new per-token rates.

## Notable Prior Milestones

### v2.0.3 — Strategic Execution + Durable Replay

- **OKR-Gated `/go` Workflow** — Strategic execution with draft → approve/deny → run semantics and persisted OKR run tracking.
- **OKR CLI & Reporting** — `codetether okr` command group: `list`, `status`, `create`, `runs`, `export`, `stats`, `report`.
- **Relay Checkpointing + Resume** — In-flight relay state checkpointed for crash recovery and exact-order continuation.
- **Event Stream Module** — Structured JSONL event sourcing with byte-range offsets for efficient replay.
- **S3/R2 Archival** — Optional archival pipeline for event streams and chat artifacts to S3-compatible storage.
- **Correlation-Rich Observability** — Audit/event models include `okr_id`, `okr_run_id`, `relay_id`, `session_id`.
- **Worker HTTP Probe Server** — `/health`, `/ready`, `/worker/status` endpoints for Kubernetes integration.

### v1.1.6-alpha-2 — Agent Bus & gRPC Transport

This release adds the inter-agent communication backbone — a broadcast-based in-process message bus and a gRPC transport layer implementing the full A2A protocol, enabling high-frequency agent-to-agent communication both locally and across the network.

- **Agent Message Bus** — Central `AgentBus` with topic-based pub/sub routing (`agent.{id}`, `task.{id}`, `swarm.{id}`, `broadcast`). Every local agent gets a zero-copy `BusHandle` for send/receive. Supports task updates, artifact sharing, tool dispatch, heartbeats, and free-form inter-agent messaging.
- **Agent Registry** — `AgentRegistry` tracks connected agents via `AgentCard`. Ephemeral card factory for short-lived sub-agents with `bus://local/{name}` URLs.
- **gRPC Transport (A2A Protocol)** — Full tonic-based gRPC server implementing all 9 A2A RPCs: `SendMessage`, `SendStreamingMessage`, `GetTask`, `CancelTask`, `TaskSubscription`, push notification config CRUD, and `GetAgentCard`. Shares state with JSON-RPC via `GrpcTaskStore`.
- **Proto-Parity A2A Types** — Complete rewrite of `a2a/types.rs` with 30+ types matching the A2A protocol spec: `SecurityScheme` (5 variants), `AgentInterface`, `AgentExtension`, `AgentCardSignature`, `OAuthFlows`, `SecurityRequirement`, and more.
- **Worker as A2A Peer** — Workers create an `AgentBus` on startup, announce readiness, and thread the bus through the full task pipeline into `SwarmExecutor`. Sub-agents communicate via the bus instead of HTTP polling.
- **Dual Transport Server** — `codetether serve` now runs both Axum (JSON-RPC) and tonic (gRPC) simultaneously. gRPC port configurable via `CODETETHER_GRPC_PORT` (default: `50051`).
- **Swarm ↔ Bus Integration** — `SwarmExecutor` accepts an optional `AgentBus` via `.with_bus()`. Swarm events (started, stage complete, subtask updates, errors, completion) are emitted on the bus alongside the TUI event channel.

### v1.1.0 — Security-First Release

- **Mandatory Authentication** — Bearer token auth middleware that **cannot be disabled**. Auto-generates HMAC-SHA256 tokens if `CODETETHER_AUTH_TOKEN` is not set. Only `/health` is exempt.
- **System-Wide Audit Trail** — Every API call, tool execution, and session event is logged to an append-only JSON Lines file. Queryable via new `/v1/audit/events` and `/v1/audit/query` endpoints.
- **Plugin Sandboxing & Code Signing** — Tool manifests are SHA-256 hashed and Ed25519 signed. Sandbox policies enforce resource limits (memory, CPU, network). Unsigned or tampered tools are rejected.
- **Kubernetes Self-Deployment** — The agent manages its own pods via the Kubernetes API. Auto-detects cluster environment, creates/updates Deployments, scales replicas, health-checks pods, and runs a reconciliation loop every 30 seconds.
- **New API Endpoints** — `/v1/audit/events`, `/v1/audit/query`, `/v1/k8s/status`, `/v1/k8s/scale`, `/v1/k8s/health`, `/v1/k8s/reconcile`

### v1.0.0

- **FunctionGemma Tool Router** — A local 270M-param model that converts text-only LLM responses into structured tool calls. Your primary LLM reasons; FunctionGemma formats. Provider-agnostic, zero-cost passthrough when not needed, safe degradation on failure.
- **RLM + FunctionGemma Integration** — The Recursive Language Model now uses structured tool dispatch instead of regex-parsed DSL. `rlm_head`, `rlm_tail`, `rlm_grep`, `rlm_count`, `rlm_slice`, `rlm_llm_query`, and `rlm_final` are proper tool definitions.
- **Marketing Theme** — New default TUI theme with cyan accents on a near-black background, matching the CodeTether site.
- **Swarm Improvements** — Validation, caching, rate limiting, and result storage modules for parallel sub-agent execution.
- **Image Tool** — New tool for image input handling.
- **27+ Built-in Tools** — File ops, LSP, code search, web fetch, shell execution, agent orchestration.

## Install

### Via npx (no Rust)

```bash
npx codetether --help
npx codetether tui
```

(This uses the npm wrapper under `npm/codetether/`, which downloads the matching prebuilt binary from GitHub Releases for your platform. Publish it to npm to make `npx codetether ...` work globally.)

**Linux / macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.ps1 | iex
```

Downloads the binary and the FunctionGemma model (~292 MB) for local tool-call routing. No Rust toolchain required.

```bash
# Skip FunctionGemma model (Linux/macOS)
curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh -s -- --no-functiongemma
```

```powershell
# Skip FunctionGemma model (Windows)
.\install.ps1 -NoFunctionGemma
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

### From crates.io

```bash
cargo install codetether-agent

# Optional: Enable hardware acceleration for the local FunctionGemma tool router 
# For Apple Silicon / Intel Mac: 
cargo install codetether-agent --features candle-accelerate 

# For Intel/AMD Linux (requires MKL libraries): 
cargo install codetether-agent --features candle-mkl 

# For Nvidia GPU: 
cargo install codetether-agent --features candle-cuda
```

## Quick Start

### 1. Configure Vault

All API keys live in HashiCorp Vault — never in config files or env vars.

```bash
export VAULT_ADDR="https://vault.example.com:8200"
export VAULT_TOKEN="hvs.your-token"

# Add a provider
vault kv put secret/codetether/providers/openrouter api_key="sk-or-v1-..."
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
codetether run "prompt"                  # Single prompt (chat, no tools)
codetether run -- "/go <task>"             # Strategic relay (OKR-gated execution)
codetether run "/autochat <task>"         # Tactical relay (fast path)
codetether swarm "complex task"          # Parallel sub-agent execution
codetether swarm "complex task" --execution-mode k8s --k8s-pod-budget 4 --k8s-image <image>  # Cluster execution
codetether ralph run --prd prd.json      # Autonomous PRD-driven development
codetether ralph create-prd --feature X  # Generate a PRD template
codetether serve --port 4096             # HTTP server (A2A + cognition APIs)
codetether worker --server URL           # A2A worker mode (+ HTTP probes on :8080 by default)
codetether okr list                      # List OKRs
codetether okr report --id <uuid>        # Show OKR or run report
codetether spawn --name planner --peer http://localhost:4096/a2a  # Spawn real A2A agent with auto-discovery
codetether config --show                 # Show config
```

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

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CODETETHER_AUTH_TOKEN` | (auto-generated) | Bearer token for API auth. If unset, HMAC-SHA256 token is generated from hostname + timestamp. |
| `KUBERNETES_SERVICE_HOST` | — | Set automatically inside K8s. Enables self-deployment features. |

### Security API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/audit/events` | List recent audit events |
| `POST` | `/v1/audit/query` | Query audit log with filters |
| `GET` | `/v1/k8s/status` | Cluster and pod status |
| `POST` | `/v1/k8s/scale` | Scale replica count |
| `POST` | `/v1/k8s/health` | Trigger health check |
| `POST` | `/v1/k8s/reconcile` | Trigger reconciliation |

## Features

### FunctionGemma Tool Router

Modern LLMs can call tools — but they're doing two jobs at once: *reasoning* about what to do, and *formatting* the structured JSON to express it. CodeTether separates these concerns.

Your primary LLM (Claude, GPT-4o, Kimi, Llama, etc.) focuses on reasoning. A tiny local model ([FunctionGemma](https://huggingface.co/google/functiongemma-270m-it), 270M params by Google) handles structured output formatting via Candle inference (~5-50ms on CPU).

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

When FunctionGemma is enabled, RLM uses structured tool dispatch instead of regex-parsed DSL — the same separation-of-concerns pattern applied to RLM's analysis loop.

```bash
codetether rlm "What are the main functions?" -f src/large_file.rs
cat logs/*.log | codetether rlm "Summarize the errors" --content -
```

#### Content Types

RLM auto-detects content type for optimized processing:

| Type | Detection | Optimization |
|------|-----------|--------------|
| `code` | Function definitions, imports | Semantic chunking by symbols |
| `logs` | Timestamps, log levels | Time-based chunking |
| `conversation` | Chat markers, turns | Turn-based chunking |
| `documents` | Markdown headers, paragraphs | Section-based chunking |

#### Example Output

```bash
$ codetether rlm "What are the 3 main functions?" -f src/chunker.rs --json
{
  "answer": "The 3 main functions are: 1) chunk_content() - splits content...",
  "iterations": 1,
  "sub_queries": 0,
  "stats": {
    "input_tokens": 322,
    "output_tokens": 235,
    "elapsed_ms": 10982
  }
}
```

### OKR-Driven Execution: From Strategy to Shipped Code

CodeTether uses **OKRs (Objectives and Key Results)** as the bridge between business strategy and autonomous agent execution. Instead of giving agents a task and hoping for the best, you state your intent, approve a structured plan, and get measurable outcomes.

#### What's an OKR?

An **Objective** is what you want to achieve. **Key Results** are the measurable outcomes that prove you achieved it.

```
Business Strategy
  └── Objective: "Make the QR-to-booking pipeline production-ready"
        ├── KR 1: Landing page loads in <2s on mobile
        ├── KR 2: QR scan → booking flow has zero broken links
        └── KR 3: Worker audit dashboard deployed and capturing data
```

OKRs aren't tasks — they're success criteria. The agent decides *how* to achieve the Key Results. You decide *what* success looks like.

#### The `/go` Lifecycle

When you type `/go` in the TUI or CLI, you're launching a **strategic execution** — not just running a prompt. Here's the full lifecycle:

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
│     └─ Deny → re-prompt with different intent                    │
│                                                                  │
│  4. Autonomous relay execution                                   │
│     └─ Swarms, tools, sequential agent turns — whatever it takes │
│                                                                  │
│  5. KR progress updates (per relay turn)                         │
│     └─ Key Results evaluated and persisted after each turn       │
│     └─ Live progress visible in TUI or via `codetether okr`      │
│                                                                  │
│  6. Completion + outcome                                         │
│     └─ Final KR outcomes recorded                                │
│     └─ Full lifecycle stored for audit and reporting             │
└──────────────────────────────────────────────────────────────────┘
```

The approve/deny gate is the critical human-in-the-loop moment. The system structures your intent into measurable outcomes; you verify that those outcomes actually match what you meant. Two minutes of strategic review, then the swarm owns execution.

#### `/go` Input Guardrails

- Use a **clean, concise objective**: `codetether run -- "/go implement X with tests"`
- Do **not** paste prior `/go` output back into `/go` (for example blocks containing `Progress:`, `Incomplete stories:`, or `Next steps:`)
- If re-running, rewrite the objective in one sentence instead of replaying logs

#### `/go` vs `/autochat`

| Command | Purpose | OKR Gate | Best For |
|---------|---------|----------|----------|
| `/go` | Strategic execution | Yes — draft → approve → run | Epics, business goals, anything you want tracked and measurable |
| `/autochat` | Tactical execution | No — runs immediately | Quick tasks, bug fixes, "just do this now" work |

Think of `/go` as deploying a mission with objectives. `/autochat` is giving a direct order. Both use the same relay execution engine underneath — the difference is whether the work is wrapped in an OKR lifecycle with approval, progress tracking, and outcome recording.

Ralph terminal outcomes are reported as:
- `Completed` — all stories passed
- `MaxIterations` — partial completion within limits
- `QualityFailed` — no stories passed quality gates

#### Long-Running Epics

OKRs naturally support long-running work:

- **Persisted to disk** — OKR runs survive restarts. If a relay crashes, resume from the last checkpoint.
- **Cumulative KR progress** — Key Results track progress across relay turns, so you see how far along an epic is while it's still running.
- **Checkpointed state** — In-flight relay state (including which KRs have been attempted, current progress, and context) is saved for crash recovery and exact-order continuation.
- **Correlation across systems** — Every audit event and event stream entry carries `okr_id`, `okr_run_id`, `relay_id`, and `session_id` fields, so you can trace any piece of work back to the strategic objective that spawned it.

#### OKR CLI

```bash
codetether okr list                     # List all OKRs and their status
codetether okr status --id <uuid>       # Detailed status of a specific OKR
codetether okr create                   # Create a new OKR interactively
codetether okr runs --id <uuid>         # List runs for an OKR
codetether okr report --id <uuid>       # Full report — objective, KRs, outcomes
codetether okr export --id <uuid>       # Export OKR data as JSON
codetether okr stats                    # Aggregate stats across all OKRs
```

#### Why This Matters

Without OKRs, autonomous agents are powerful but opaque — you tell them what to do and hope for the best. With OKRs, every piece of autonomous work has:

- A **stated objective** (what are we trying to achieve?)
- **Measurable key results** (how do we know we achieved it?)
- An **approval gate** (did a human agree this plan makes sense?)
- **Progress tracking** (how far along are we?)
- A **completion record** (what was the outcome?)

That's the difference between "I ran some agents" and "I deployed a strategic initiative with measurable outcomes." It's how you go from managing tasks to directing intent — your job becomes approving the right objectives and letting the swarms handle everything else.

### Swarm: Parallel Sub-Agent Execution

Decomposes complex tasks into subtasks and executes them concurrently with real-time progress in the TUI.

```bash
codetether swarm "Implement user auth with tests and docs"
codetether swarm "Refactor the API layer" --strategy domain --max-subagents 8
codetether swarm "Ship feature X end-to-end" --execution-mode k8s --k8s-pod-budget 6 --k8s-image us-central1-docker.pkg.dev/<project>/<repo>/<image>@sha256:<digest>
```

Strategies: `auto` (default), `domain`, `data`, `stage`, `none`.

Execution modes:

- `local` (default): sub-agents run as local async tasks.
- `k8s`: sub-agents run as isolated Kubernetes pods, with deterministic collapse-based pruning/promotion applied during execution.

Kubernetes mode notes:

- Use `--k8s-pod-budget` to bound concurrent speculative pods.
- Use `--k8s-image` to choose the exact sub-agent image.
- Sub-agent pod lifecycle is managed automatically (spawn, monitor, terminate, cleanup).
- Runtime compatibility: newer images with `swarm-subagent` use the native remote-subtask protocol.
- Runtime compatibility: older images fall back automatically to `codetether run`.
- Forwarded env vars (when set): `VAULT_ADDR`, `VAULT_TOKEN`, `VAULT_MOUNT`, `VAULT_SECRETS_PATH`, `VAULT_NAMESPACE`, `CODETETHER_AUTH_TOKEN`.

### Ralph: Autonomous PRD-Driven Development

Give it a spec, watch it work story by story. Each iteration is a fresh agent with full tool access. Memory persists via git history, `progress.txt`, and the PRD file.

```bash
codetether ralph create-prd --feature "User Auth" --project-name my-app
codetether ralph run --prd prd.json --max-iterations 10
```

### TUI

The terminal UI includes a webview layout, model selector, session picker, swarm view with per-agent detail, Ralph view with per-story progress, and theme support with hot-reload.

**Slash Commands**: `/go`, `/autochat`, `/swarm`, `/ralph`, `/model`, `/sessions`, `/resume`, `/new`, `/webview`, `/classic`, `/inspector`, `/refresh`, `/view`

**Keyboard**: `Ctrl+M` model selector, `Ctrl+B` toggle layout, `Ctrl+S`/`F2` swarm view, `Tab` switch agents, `Alt+j/k` scroll, `?` help, plus `A`/`D` for OKR approve/deny prompts in `/go` flow

## Providers

| Provider | Default Model | Notes |
|----------|---------------|-------|
| `zai` | `glm-5` | Z.AI flagship — GLM-5 agentic coding (200K ctx) |
| `moonshotai` | `kimi-k2.5` | Default — excellent for coding |
| `github-copilot` | `claude-opus-4` | GitHub Copilot models |
| `openrouter` | `stepfun/step-3.5-flash:free` | Access to many models |
| `google` | `gemini-2.5-pro` | Google AI |
| `anthropic` | `claude-sonnet-4-20250514` | Direct or via Azure |
| `stepfun` | `step-3.5-flash` | Chinese reasoning model |
| `vertex-glm` | `zai-org/glm-5-maas` | GLM-5 via Google Cloud Vertex AI (service account JWT auth) |
| `bedrock` | — | Amazon Bedrock Converse API |

All keys stored in Vault at `secret/codetether/providers/<name>`.

#### Vertex GLM Setup

The `vertex-glm` provider uses a GCP service account for authentication (no `gcloud` CLI required). Store the service account JSON key and project ID in Vault:

```bash
SA_JSON=$(cat /path/to/service-account-key.json)
vault kv put secret/codetether/providers/vertex-glm \
  project_id="your-gcp-project" \
  service_account_json="$SA_JSON"
```

The service account needs the **Vertex AI User** role (`roles/aiplatform.user`). Tokens are cached and auto-refreshed.

## Tools

27+ tools across file operations (`read_file`, `write_file`, `edit`, `multiedit`, `apply_patch`, `glob`, `list_dir`), code intelligence (`lsp`, `grep`, `codesearch`), execution (`bash`, `batch`, `task`), web (`webfetch`, `websearch`), and agent orchestration (`ralph`, `rlm`, `prd`, `swarm`, `todo_read`, `todo_write`, `question`, `skill`, `plan_enter`, `plan_exit`).

## MCP Server (VS Code / Claude Desktop)

CodeTether exposes 26 tools via the [Model Context Protocol](https://modelcontextprotocol.io/) over stdio. This lets AI clients (GitHub Copilot in VS Code, Claude Desktop, etc.) call CodeTether tools directly.

### VS Code (Workspace-Level — Recommended)

When using VS Code Remote SSH, the extension host runs on the remote machine. Add a `.vscode/mcp.json` to your workspace:

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

Reload VS Code — the 26 tools appear in the MCP panel automatically.

### Claude Desktop (Global Config)

Edit `%APPDATA%\Claude\claude_desktop_config.json` (Windows) or `~/.config/Claude/claude_desktop_config.json` (Linux/macOS):

```json
{
  "mcpServers": {
    "codetether": {
      "command": "/path/to/codetether",
      "args": ["mcp", "serve"],
      "env": {
        "RUST_LOG": "error"
      }
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

### Exposed Tools (26)

| Category | Tools |
|----------|-------|
| **File Ops** | `read`, `write`, `edit`, `multiedit`, `apply_patch`, `glob`, `list` |
| **Search** | `grep`, `codesearch` |
| **Execution** | `bash`, `task` |
| **Code Intelligence** | `lsp` |
| **Web** | `webfetch`, `websearch` |
| **Agent Orchestration** | `agent`, `swarm_execute`, `relay_autochat`, `go` |
| **Planning** | `ralph`, `prd`, `okr`, `todoread`, `todowrite` |
| **Knowledge** | `memory`, `skill`, `mcp` (bridge to other MCP servers) |

### CLI Helpers

```bash
codetether mcp list-tools          # List available MCP tools
codetether mcp list-tools --json   # JSON output
codetether mcp serve               # Start stdio MCP server
codetether mcp serve --bus-url URL # With agent bus integration
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    CodeTether Platform                       │
│                 (A2A Server at api.codetether.run)           │
└───────────────┬───────────────────────┬─────────────────────┘
                │ SSE/JSON-RPC          │ gRPC (A2A proto)
                ▼                       ▼
┌─────────────────────────────────────────────────────────────┐
│                    codetether-agent                          │
│                                                             │
│   ┌───────────────────────────────────────────────────┐     │
│   │              Agent Message Bus                     │     │
│   │   (broadcast pub/sub, topic routing, BusHandle)    │     │
│   └──┬──────────┬──────────┬──────────┬───────────────┘     │
│      │          │          │          │                      │
│   ┌──┴───┐  ┌──┴───┐  ┌──┴───┐  ┌──┴────────┐             │
│   │ A2A  │  │ Swarm│  │ Tool │  │  Provider  │             │
│   │Worker│  │ Exec │  │System│  │   Layer    │             │
│   └──┬───┘  └──┬───┘  └──┬───┘  └──┬────────┘             │
│      │         │         │         │                        │
│   ┌──┴─────────┴─────────┴─────────┴──┐                    │
│   │         Agent Registry             │                    │
│   │  (AgentCard, ephemeral sub-agents) │                    │
│   └───────────────────────────────────┘                    │
│                                                             │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│   │JSON-RPC  │  │ gRPC     │  │ Auth     │  │ Audit    │  │
│   │(Axum)    │  │ (Tonic)  │  │ (Bearer) │  │ (JSONL)  │  │
│   │:4096     │  │ :50051   │  │ Mandatory│  │ Append   │  │
│   └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
│   ┌──────────┐  ┌──────────┐  ┌──────────────────────┐    │
│   │ Sandbox  │  │ K8s Mgr  │  │  HashiCorp Vault     │    │
│   │ (Ed25519)│  │ (Deploy) │  │  (API Keys)          │    │
│   └──────────┘  └──────────┘  └──────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## A2A Protocol

Built for Agent-to-Agent communication with dual transports and a shared in-process bus:

- **Worker mode** — Connect to the CodeTether platform and process tasks. Creates a local `AgentBus` for sub-agent coordination.
- **Server mode** — Accept tasks from other agents (`codetether serve`) via JSON-RPC (Axum) and gRPC (Tonic) simultaneously.
- **Spawn mode** — Launch a standalone A2A peer (`codetether spawn`) that serves its own `AgentCard`, auto-registers on the local agent bus, and continuously discovers peer agents.
- **Bus mode** — In-process pub/sub for zero-latency communication between local agents, swarm sub-agents, and tool dispatch.
- **Cognition APIs** — Perpetual persona swarms with SSE event stream, spawn/reap control, and lineage graph.

### Transports

| Transport | Port | Use Case |
|-----------|------|----------|
| JSON-RPC (Axum) | `4096` (default) | REST API, SSE streams, `/.well-known/agent.json` |
| gRPC (Tonic) | `50051` (default) | High-frequency A2A protocol RPCs, streaming |
| In-Process Bus | — | Local sub-agents, swarm coordination, tool dispatch |

### gRPC RPCs (A2A Protocol)

| RPC | Description |
|-----|-------------|
| `SendMessage` | Submit a task/message to the agent |
| `SendStreamingMessage` | Submit with server-streaming status updates |
| `GetTask` | Retrieve task by ID |
| `CancelTask` | Cancel a running task |
| `TaskSubscription` | Subscribe to task status updates (server-stream) |
| `CreateTaskPushNotificationConfig` | Register push notification endpoint |
| `GetTaskPushNotificationConfig` | Get push notification config |
| `ListTaskPushNotificationConfig` | List all push configs for a task |
| `DeleteTaskPushNotificationConfig` | Remove a push notification config |
| `GetAgentCard` | Retrieve the agent's capability card |

### Agent Bus Topics

| Topic Pattern | Semantics |
|---------------|-----------|
| `agent.{id}` | Messages *to* a specific agent |
| `agent.{id}.events` | Events *from* a specific agent |
| `task.{id}` | All updates for a task |
| `swarm.{id}` | Swarm-level coordination |
| `broadcast` | Global announcements |
| `results.{key}` | Shared result publication |
| `tools.{name}` | Tool-specific channels |

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CODETETHER_GRPC_PORT` | `50051` | gRPC server port (used alongside Axum HTTP) |
| `CODETETHER_A2A_PEERS` | — | Comma-separated peer seed URLs used by `codetether spawn` discovery loop |

### AgentCard

When running as a server, the agent exposes its capabilities via `/.well-known/agent.json`:

```json
{
  "name": "CodeTether Agent",
  "description": "A2A-native AI coding agent",
  "version": "4.0.0",
  "preferred_transport": "GRPC",
  "skills": [
    { "id": "code-generation", "name": "Code Generation" },
    { "id": "code-review", "name": "Code Review" },
    { "id": "debugging", "name": "Debugging" }
  ]
}
```

### Perpetual Persona Swarms API (Phase 0)

When running `codetether serve`, the agent also exposes cognition + swarm control APIs:

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/cognition/start` | Start perpetual cognition loop |
| `POST` | `/v1/cognition/stop` | Stop cognition loop |
| `GET` | `/v1/cognition/status` | Runtime status and buffer metrics |
| `GET` | `/v1/cognition/stream` | SSE stream of thought events |
| `GET` | `/v1/cognition/snapshots/latest` | Latest compressed memory snapshot |
| `POST` | `/v1/swarm/personas` | Create a root persona |
| `POST` | `/v1/swarm/personas/{id}/spawn` | Spawn child persona |
| `POST` | `/v1/swarm/personas/{id}/reap` | Reap a persona (optional cascade) |
| `GET` | `/v1/swarm/lineage` | Current persona lineage graph |

`/v1/cognition/start` auto-seeds a default `root-thinker` persona when no personas exist, unless a `seed_persona` is provided.

### Worker Cognition Sharing (Social-by-Default)

When running in worker mode, CodeTether can include cognition status and a short latest-thought summary in worker heartbeats so upstream systems can monitor active reasoning state.

- Default: enabled
- Privacy control: disable any time with `CODETETHER_WORKER_COGNITION_SHARE_ENABLED=false`
- Safety: summary text is truncated before transmission (default `480` chars)

```bash
# Disable upstream thought sharing
export CODETETHER_WORKER_COGNITION_SHARE_ENABLED=false

# Point worker to a non-default local cognition API
export CODETETHER_WORKER_COGNITION_SOURCE_URL=http://127.0.0.1:4096

# Keep status sharing but disable thought text
export CODETETHER_WORKER_COGNITION_INCLUDE_THOUGHTS=false
```

| Variable | Default | Description |
|----------|---------|-------------|
| `CODETETHER_WORKER_COGNITION_SHARE_ENABLED` | `true` | Enable cognition payload in worker heartbeat |
| `CODETETHER_WORKER_COGNITION_SOURCE_URL` | `http://127.0.0.1:4096` | Local cognition API base URL |
| `CODETETHER_WORKER_COGNITION_INCLUDE_THOUGHTS` | `true` | Include latest thought summary |
| `CODETETHER_WORKER_COGNITION_THOUGHT_MAX_CHARS` | `480` | Max chars for latest thought summary |
| `CODETETHER_WORKER_COGNITION_TIMEOUT_MS` | `2500` | Timeout for local cognition API reads |

See `docs/perpetual_persona_swarms.md` for request/response contracts.

### CUDA Build/Deploy Helpers

- `make build-cuda` — Build a CUDA-enabled binary locally
- `make deploy-spike2-cuda` — Sync source to `spike2`, build with `--features candle-cuda`, install, and restart service
- `make status-spike2-cuda` — Check service status, active Candle device config, and GPU usage on `spike2`

## Dogfooding: Self-Implementing Agent

CodeTether implemented its own features using `ralph` and `swarm`.

### What We Accomplished

Using `ralph` and `swarm`, the agent autonomously implemented:

**LSP Client Implementation (10 stories)**:
- US-001: LSP Transport Layer - stdio implementation
- US-002: JSON-RPC Message Framework
- US-003: LSP Initialize Handshake
- US-004: Text Document Synchronization - didOpen
- US-005: Text Document Synchronization - didChange
- US-006: Text Document Completion
- US-007: Text Document Hover
- US-008: Text Document Definition
- US-009: LSP Shutdown and Exit
- US-010: LSP Client Configuration and Server Management

**Missing Features (10 stories)**:
- MF-001: External Directory Tool
- MF-002: RLM Pool - Connection Pooling
- MF-003: Truncation Utilities
- MF-004: LSP Full Integration - Server Management
- MF-005: LSP Transport - stdio Communication
- MF-006: LSP Requests - textDocument/definition
- MF-007: LSP Requests - textDocument/references
- MF-008: LSP Requests - textDocument/hover
- MF-009: LSP Requests - textDocument/completion
- MF-010: RLM Router Enhancement

### Results

| Metric | Value |
|--------|-------|
| **Total User Stories** | 20 |
| **Stories Passed** | 20 (100%) |
| **Total Iterations** | 20 |
| **Quality Checks Per Story** | 4 (check, clippy, test, build) |
| **Lines of Code Generated** | ~6,000+ |
| **Time to Complete** | ~30 minutes |
| **Model Used** | Kimi K2.5 (Moonshot AI) |

### Efficiency Comparison

| Approach | Time | Cost | Notes |
|----------|------|------|-------|
| **Manual Development** | 80 hours | $8,000 | Senior dev @ $100/hr, 50-100 LOC/day |
| **agent + subagents** | 100 min | ~$11.25 | Bun runtime, Kimi K2.5 (same model) |
| **codetether swarm** | 29.5 min | $3.75 | Native Rust, Kimi K2.5 |

**vs Manual**: 163x faster, 2133x cheaper
**vs agent**: 3.4x faster, ~3x cheaper (same Kimi K2.5 model)

Key advantages over agent subagents (model parity):
- Native Rust binary (13ms startup vs 25-50ms Bun)
- Direct API calls vs TypeScript HTTP overhead
- PRD-driven state in files vs subagent process spawning
- ~3x fewer tokens due to reduced subagent initialization overhead

**Note**: Both have LLM-based compaction. The efficiency gain comes from PRD-driven architecture (state in prd.json + progress.txt) vs. spawning subprocesses with rebuilt context.

### How to Replicate

```bash
# 1. Create a PRD for your feature
cat > prd.json << 'EOF'
{
  "project": "my-project",
  "feature": "My Feature",
  "quality_checks": {
    "typecheck": "cargo check",
    "test": "cargo test",
    "lint": "cargo clippy",
    "build": "cargo build --release"
  },
  "user_stories": [
    {
      "id": "US-001",
      "title": "First Story",
      "description": "Implement the first piece",
      "acceptance_criteria": ["Compiles", "Tests pass"],
      "priority": 1,
      "depends_on": [],
      "passes": false
    }
  ]
}
EOF

# 2. Run Ralph
codetether ralph run -p prd.json -m "kimi-k2.5" --max-iterations 10

# 3. Watch as your feature gets implemented autonomously
```

### Why This Matters

1. **Proof of Capability**: The agent can implement non-trivial features end-to-end
2. **Quality Assurance**: Every story passes cargo check, clippy, test, and build
3. **Autonomous Operation**: No human intervention during implementation
4. **Reproducible Process**: PRD-driven development is structured and repeatable
5. **Self-Improvement**: The agent literally improved itself

## Performance: Why Rust Over Bun/TypeScript

CodeTether Agent is written in Rust for measurable performance advantages over JavaScript/TypeScript runtimes like Bun:

### Benchmark Results

| Metric | CodeTether (Rust) | agent (Bun) | Advantage |
|--------|-------------------|----------------|-----------|
| **Binary Size** | 12.5 MB | ~90 MB (bun + deps) | **7.2x smaller** |
| **Startup Time** | 13 ms | 25-50 ms | **2-4x faster** |
| **Memory (idle)** | ~15 MB | ~50-80 MB | **3-5x less** |
| **Memory (swarm, 10 agents)** | ~45 MB | ~200+ MB | **4-5x less** |
| **Process Spawn** | 1.5 ms | 5-10 ms | **3-7x faster** |
| **Cold Start (container)** | ~50 ms | ~200-500 ms | **4-10x faster** |

### Why This Matters for Sub-Agents

1. **Lower Memory Per Agent**: With 3-5x less memory per agent, you can run more concurrent sub-agents on the same hardware. A 4GB container can run ~80 Rust sub-agents vs ~15-20 Bun sub-agents.
2. **Faster Spawn Time**: Sub-agents spawn in 1.5ms vs 5-10ms. For a swarm of 100 agents, that's 150ms vs 500-1000ms just in spawn overhead.
3. **No GC Pauses**: Rust has no garbage collector. JavaScript/Bun has GC pauses that can add latency spikes of 10-50ms during high-memory operations.
4. **True Parallelism**: Rust's tokio runtime uses OS threads with work-stealing. Bun uses a single-threaded event loop that can bottleneck on CPU-bound decomposition.
5. **Smaller Attack Surface**: Smaller binary = fewer dependencies = smaller CVE surface. Critical for agents with shell access.

### Resource Efficiency for Swarm Workloads

```
┌─────────────────────────────────────────────────────────────────┐
│                    Memory Usage Comparison                      │
│                                                                 │
│  Sub-Agents    CodeTether (Rust)       agent (Bun)           │
│  ────────────────────────────────────────────────────────────── │
│       1            15 MB                   60 MB                │
│       5            35 MB                  150 MB                │
│      10            55 MB                  280 MB                │
│      25           105 MB                  650 MB                │
│      50           180 MB                 1200 MB                │
│     100           330 MB                 2400 MB                │
│                                                                 │
│  At 100 sub-agents: Rust uses 7.3x less memory                 │
└─────────────────────────────────────────────────────────────────┘
```

### Real-World Impact

For a typical swarm task (e.g., "Implement feature X with tests"):

| Scenario | CodeTether | agent (Bun) |
|----------|------------|----------------|
| Task decomposition | 50ms | 150ms |
| Spawn 5 sub-agents | 8ms | 35ms |
| Peak memory | 45 MB | 180 MB |
| Total overhead | ~60ms | ~200ms |

**Result**: 3.3x faster task initialization, 4x less memory, more capacity for actual AI inference.

### Measured: Dogfooding Task (20 User Stories)

Actual resource usage from implementing 20 user stories autonomously:

```
┌─────────────────────────────────────────────────────────────────┐
│           Dogfooding Task: 20 Stories, Same Model (Kimi K2.5)   │
│                                                                 │
│  Metric              CodeTether           agent (estimated)  │
│  ────────────────────────────────────────────────────────────── │
│  Total Time          29.5 min             100 min (3.4x slower) │
│  Wall Clock          1,770 sec            6,000 sec             │
│  Iterations          20                   20                    │
│  Spawn Overhead      20 × 1.5ms = 30ms   20 × 7.5ms = 150ms   │
│  Startup Overhead    20 × 13ms = 260ms   20 × 37ms = 740ms    │
│  Peak Memory         ~55 MB               ~280 MB               │
│  Tokens Used         500K                 ~1.5M (subagent init) │
│  Token Cost          $3.75                ~$11.25               │
│                                                                 │
│  Total Overhead      290ms                890ms (3.1x more)     │
│  Memory Efficiency   5.1x less peak RAM                         │
│  Cost Efficiency     ~3x cheaper                                │
└─────────────────────────────────────────────────────────────────┘
```

**Computation Notes**:
- Spawn overhead: `iterations × spawn_time` (1.5ms Rust vs 7.5ms Bun avg)
- Startup overhead: `iterations × startup_time` (13ms Rust vs 37ms Bun avg)
- Token difference: agent has compaction, but subagent spawns rebuild system prompt + context each time (~3x more tokens)
- Memory: Based on 10-agent swarm profile (55 MB vs 280 MB)
- Cost: Same Kimi K2.5 pricing, difference is from subagent initialization overhead

**Note**: agent uses LLM-based compaction for long sessions (similar to codetether). The token difference comes from subagent process spawning overhead, not lack of context management.

### Benchmark Methodology

Run benchmarks yourself:

```bash
./script/benchmark.sh
```

Benchmarks performed on:
- Ubuntu 24.04, x86_64
- 48 CPU threads, 32GB RAM
- Rust 1.85, Bun 1.x
- HashiCorp Vault for secrets

## Configuration

`~/.config/codetether-agent/config.toml`:

```toml
[default]
provider = "anthropic"
model = "claude-sonnet-4-20250514"

[ui]
theme = "marketing"   # marketing (default), dark, light, solarized-dark, solarized-light

[session]
auto_save = true
```

### Vault Environment Variables

| Variable | Description |
|----------|-------------|
| `VAULT_ADDR` | Vault server address |
| `VAULT_TOKEN` | Authentication token |
| `VAULT_MOUNT` | KV mount path (default: `secret`) |
| `VAULT_SECRETS_PATH` | Provider secrets prefix (default: `codetether/providers`) |

## Crash Reporting (Opt-In)

Disabled by default. Captures panic info on next startup — no source files or API keys included.

```bash
codetether config --set telemetry.crash_reporting=true
codetether config --set telemetry.crash_reporting=false
```

## Performance

| Metric | Value |
|--------|-------|
| **Startup** | 13ms |
| **Memory (idle)** | ~15 MB |
| **Memory (10-agent swarm)** | ~55 MB |
| **Binary size** | ~12.5 MB |

Written in Rust with tokio — true parallelism, no GC pauses, native performance.

## Development

```bash
cargo build                  # Debug build
cargo build --release        # Release build
cargo test                   # Run tests
cargo clippy --all-features  # Lint
cargo fmt                    # Format
./script/benchmark.sh        # Run benchmarks
```

## License

MIT
# Test commit script fix
