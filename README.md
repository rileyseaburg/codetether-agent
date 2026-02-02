# CodeTether Agent

A high-performance AI coding agent with first-class A2A (Agent-to-Agent) protocol support, written in Rust. Part of the CodeTether ecosystem.

## Features

- **A2A-Native**: Built from the ground up for the A2A protocol - works as a worker agent for the CodeTether platform
- **AI-Powered Coding**: Intelligent code assistance using multiple AI providers (OpenAI, Anthropic, Google, DeepSeek, etc.)
- **Secure Secrets**: All API keys loaded exclusively from HashiCorp Vault - no environment variable secrets
- **Interactive TUI**: Beautiful terminal interface built with Ratatui
- **Multiple Agents**: Specialized agents for different workflows (build, plan, explore)
- **Tool System**: Extensible tool system for file operations, search, editing, and shell commands
- **Session Management**: Persistent session history with git-aware storage
- **High Performance**: Written in Rust for maximum efficiency and reliability

## Installation

```bash
cargo install codetether-agent
```

Or build from source:

```bash
git clone https://github.com/codetether/codetether-agent
cd codetether-agent
cargo build --release
```

## Quick Start

### 1. Configure HashiCorp Vault

All API keys are stored in HashiCorp Vault for security. Set up your Vault connection:

```bash
export VAULT_ADDR="https://vault.example.com:8200"
export VAULT_TOKEN="hvs.your-token"
```

Store your provider API keys in Vault:

```bash
# OpenAI
vault kv put secret/codetether/providers/openai api_key="sk-..."

# Anthropic
vault kv put secret/codetether/providers/anthropic api_key="sk-ant-..."

# Google AI
vault kv put secret/codetether/providers/google api_key="AIza..."
```

### 2. Connect to CodeTether Platform

```bash
# Connect as a worker to the CodeTether A2A server
codetether --server https://api.codetether.run

# Or with authentication
codetether --server https://api.codetether.run --token your-worker-token
```

### 3. Or Use Interactive Mode

```bash
# Start the TUI in current directory
codetether tui

# Start in a specific project
codetether tui /path/to/project
```

## Usage

### Default Mode: A2A Worker

By default, `codetether` runs as an A2A worker that connects to the CodeTether platform:

```bash
# Connect to CodeTether platform
codetether --server https://api.codetether.run

# With custom worker name
codetether --server https://api.codetether.run --name "my-dev-machine"
```

Environment variables:
- `CODETETHER_SERVER` - A2A server URL
- `CODETETHER_TOKEN` - Authentication token
- `CODETETHER_WORKER_NAME` - Worker name

### Interactive TUI

```bash
codetether tui
```

### Non-Interactive Mode

```bash
# Run a single prompt
codetether run "implement the todo feature"

# Continue from last session
codetether run --continue "add tests for the new feature"
```

### HTTP Server

```bash
# Start the API server
codetether serve --port 4096
```

### Configuration Management

```bash
# Show current config
codetether config --show

# Initialize default config
codetether config --init
```

## Configuration

Configuration is stored in `~/.config/codetether-agent/config.toml`:

```toml
[default]
provider = "anthropic"
model = "claude-sonnet-4-20250514"

[a2a]
enabled = true
auto_connect = true

[ui]
theme = "dark"

[session]
auto_save = true
```

**Note:** API keys are NOT stored in config files. They must be stored in HashiCorp Vault.

## HashiCorp Vault Setup

### Vault Secret Structure

```
secret/codetether/providers/
├── openai       → { "api_key": "sk-...", "organization": "org-..." }
├── anthropic    → { "api_key": "sk-ant-..." }
├── google       → { "api_key": "AIza..." }
├── deepseek     → { "api_key": "..." }
└── ...
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `VAULT_ADDR` | Vault server address (e.g., `https://vault.example.com:8200`) |
| `VAULT_TOKEN` | Vault authentication token |
| `VAULT_MOUNT` | KV secrets engine mount path (default: `secret`) |
| `VAULT_SECRETS_PATH` | Path prefix for provider secrets (default: `codetether/providers`) |

### Using Vault Agent

For production, use Vault Agent for automatic token renewal:

```hcl
# vault-agent.hcl
vault {
  address = "https://vault.example.com:8200"
}

auto_auth {
  method "kubernetes" {
    mount_path = "auth/kubernetes"
    config = {
      role = "codetether-agent"
    }
  }

  sink "file" {
    config = {
      path = "/tmp/vault-token"
    }
  }
}
```

## Agents

### Build Agent

Full access to development tools. Can read, write, edit files and execute commands.

### Plan Agent

Read-only access for analysis and exploration. Perfect for understanding codebases before making changes.

### Explore Agent

Specialized for code navigation and discovery.

## Tools

| Tool | Description |
|------|-------------|
| `read_file` | Read file contents |
| `write_file` | Write content to files |
| `list_dir` | List directory contents |
| `glob` | Find files by pattern |
| `grep` | Search file contents with regex |
| `edit` | Apply search/replace patches |
| `bash` | Execute shell commands |

## A2A Protocol

CodeTether Agent is built for the A2A (Agent-to-Agent) protocol:

- **Worker Mode** (default): Connect to the CodeTether platform and process tasks
- **Server Mode**: Accept tasks from other agents (`codetether serve`)
- **Client Mode**: Dispatch tasks to other A2A agents

### AgentCard

When running as a server, the agent exposes its capabilities via `/.well-known/agent.json`:

```json
{
  "name": "CodeTether Agent",
  "description": "A2A-native AI coding agent",
  "version": "0.1.0",
  "skills": [
    { "id": "code-generation", "name": "Code Generation" },
    { "id": "code-review", "name": "Code Review" },
    { "id": "debugging", "name": "Debugging" }
  ]
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   CodeTether Platform                    │
│                  (A2A Server at api.codetether.run)     │
└────────────────────────┬────────────────────────────────┘
                         │ SSE/JSON-RPC
                         ▼
┌─────────────────────────────────────────────────────────┐
│                   codetether-agent                       │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐    │
│  │ A2A     │  │ Agent   │  │ Tool    │  │ Provider│    │
│  │ Worker  │  │ System  │  │ System  │  │ Layer   │    │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘    │
│       │            │            │            │          │
│       └────────────┴────────────┴────────────┘          │
│                         │                                │
│  ┌──────────────────────┴──────────────────────────┐    │
│  │              HashiCorp Vault                      │    │
│  │         (API Keys & Secrets)                     │    │
│  └──────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## Development

```bash
# Run in development mode
cargo run -- --server http://localhost:8080

# Run tests
cargo test

# Build release binary
cargo build --release
```

## License

MIT
