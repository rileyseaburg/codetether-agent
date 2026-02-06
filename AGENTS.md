# AGENTS.md

Instructions for AI coding agents working on CodeTether Agent.

## Setup Commands

```bash
# Install dependencies and build
cargo build

# Run tests
cargo test

# Run with clippy lints
cargo clippy --all-features

# Build release binary
cargo build --release

# Install locally
cargo install --path .

# Run the TUI
codetether tui

# Run a single prompt
codetether run "your message"

# Run as A2A worker (connects to server, registers codebases)
codetether worker --server http://localhost:8001 --codebases /path/to/project --auto-approve safe

# Deploy as a service (one-command wrapper)
./deploy-worker.sh --codebases /path/to/project
```

## Code Style

- Rust 2024 edition with `rust-version = "1.85"`
- Use `anyhow::Result` for fallible functions in application code
- Use `thiserror` for library error types
- Prefer `tracing` over `println!` for logging
- Always use `tracing::info!`, `tracing::warn!`, `tracing::error!` with structured fields, e.g., `tracing::info!(tool = %name, "Executing tool")`
- Inline format args when possible: `format!("{name}")` not `format!("{}", name)`
- Collapse if statements per clippy `collapsible_if`
- Use method references over closures when possible: `.map(String::as_str)` not `.map(|s| s.as_str())`
- Prefer exhaustive `match` statements over wildcard arms when the enum is small

## Project Structure

```
src/
├── main.rs          # CLI entry point (clap-based)
├── lib.rs           # Library root - re-exports all modules
├── a2a/             # A2A protocol client/server/worker
├── agent/           # Agent definitions (builtin agents)
├── cli/             # CLI commands (run, tui, serve, ralph, swarm, etc.)
├── config/          # Configuration loading
├── mcp/             # MCP protocol implementation
├── provider/        # LLM provider implementations
├── ralph/           # Autonomous PRD-driven development loop
├── rlm/             # Recursive Language Model processing
├── secrets/         # HashiCorp Vault secrets management
├── server/          # HTTP server (axum)
├── session/         # Conversation session management
├── swarm/           # Parallel sub-agent orchestration
├── tool/            # Tool implementations (24+ tools)
├── tui/             # Terminal UI (ratatui + crossterm)
└── worktree/        # Git worktree management for isolation
```

## Adding a New Tool

1. Create `src/tool/your_tool.rs`:

```rust
use super::{Tool, ToolDefinition, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourToolInput {
    pub required_field: String,
    #[serde(default)]
    pub optional_field: Option<String>,
}

pub struct YourTool;

#[async_trait]
impl Tool for YourTool {
    fn name(&self) -> &'static str {
        "your_tool"
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name().to_string(),
            description: "What the tool does".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "required_field": {
                        "type": "string",
                        "description": "Description of this field"
                    },
                    "optional_field": {
                        "type": "string",
                        "description": "Optional field description"
                    }
                },
                "required": ["required_field"]
            }),
        }
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let params: YourToolInput = serde_json::from_value(input)?;

        // Tool implementation here

        Ok(ToolResult {
            output: "Result text".to_string(),
            success: true,
        })
    }
}
```

2. Register in `src/tool/mod.rs`:
   - Add `pub mod your_tool;`
   - Add to `ToolRegistry::new()` or `ToolRegistry::with_provider_arc()`

## Adding a New Provider

1. Create `src/provider/your_provider.rs` implementing `Provider` trait:

```rust
#[async_trait]
impl Provider for YourProvider {
    fn name(&self) -> &str { "yourprovider" }
    fn models(&self) -> Vec<String> { vec!["model-1".into()] }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    async fn stream(&self, request: CompletionRequest) -> Result<Pin<Box<dyn Stream<Item = Result<StreamChunk>>>>>;
}
```

2. Register in `src/provider/mod.rs` within `ProviderRegistry::from_vault()`

3. Add Vault secret path: `secret/codetether/providers/yourprovider`

## Secrets Management

**CRITICAL**: Never hardcode API keys. All secrets come from HashiCorp Vault.

```rust
// Good - load from Vault
let registry = ProviderRegistry::from_vault().await?;

// Bad - never do this
let api_key = "sk-..."; // NEVER
```

Environment variables for Vault:
- `VAULT_ADDR` - Vault server address
- `VAULT_TOKEN` - Authentication token
- `VAULT_MOUNT` - KV mount (default: `secret`)
- `VAULT_SECRETS_PATH` - Path prefix (default: `codetether/providers`)

## Testing

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test --lib session

# Run a specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Test Patterns

- Use `#[tokio::test]` for async tests
- Use `tempfile::tempdir()` for file system tests
- Mock providers for unit tests, use real providers in integration tests
- Integration tests go in `tests/` directory

## TUI Development

The TUI uses ratatui 0.30.0 + crossterm 0.29.0.

### Styling Conventions

```rust
// Good - use Stylize helpers
"text".dim()
"text".cyan().bold()
vec!["prefix".dim(), "content".into()].into()

// Avoid - verbose style construction
Span::styled("text", Style::default().fg(Color::Cyan))
```

### Color Guidelines

- User messages: default foreground
- Assistant messages: Cyan (not Blue - better readability)
- System/status: Dim
- Errors: Red
- Success: Green

### Key Bindings

- `Ctrl+C` / `Ctrl+Q`: Quit
- `?`: Help
- `Tab`: Switch agent
- `↑↓`: Navigate/scroll
- `Enter`: Submit input

## Swarm Sub-Agents

When implementing swarm sub-agents:

1. **Filter interactive tools**: Sub-agents must be autonomous - filter out `question` tool:
   ```rust
   .filter(|t| t.name != "question")
   ```

2. **Worktree isolation**: Use `inject_workspace_stub()` for Cargo workspace isolation:
   ```rust
   mgr.inject_workspace_stub(&worktree_path)?;
   ```

3. **Token limits**: Sub-agents may hit context limits. Handle gracefully with truncation.

## Ralph (Autonomous Loop)

Ralph implements PRD-driven development. Key files:
- `src/ralph/ralph_loop.rs` - Main loop
- `src/ralph/types.rs` - PRD structures

### PRD Structure

```json
{
  "project": "project-name",
  "feature": "Feature Name",
  "quality_checks": {
    "typecheck": "cargo check",
    "lint": "cargo clippy",
    "test": "cargo test",
    "build": "cargo build --release"
  },
  "user_stories": [
    {
      "id": "US-001",
      "title": "Story title",
      "description": "What to implement",
      "acceptance_criteria": ["Criterion 1", "Criterion 2"],
      "priority": 1,
      "depends_on": [],
      "passes": false
    }
  ]
}
```

### Memory Persistence

Ralph uses file-based memory (not context accumulation):
- `progress.txt` - Agent writes learnings/blockers
- `prd.json` - Tracks pass/fail status
- Git history - Shows what changed per iteration

## Message Roles

When building conversation messages:

```rust
// User message
Message { role: Role::User, content: vec![ContentPart::Text { text }] }

// Assistant message
Message { role: Role::Assistant, content: vec![ContentPart::Text { text }] }

// Tool result (MUST use Role::Tool, not Role::User)
Message { role: Role::Tool, content: vec![ContentPart::ToolResult { tool_call_id, content }] }
```

## Common Pitfalls

1. **Tool results must use `Role::Tool`** - Using `Role::User` causes API errors with tool call validation

2. **Kimi K2.5 requires `temperature=1.0`** - Other temperatures may cause issues

3. **Worktrees need Cargo workspace isolation** - Use `inject_workspace_stub()` to prepend `[workspace]` to Cargo.toml

4. **TodoStatus enum has aliases** - Accepts both `inprogress` and `in_progress` variants

5. **Session must call `.save()` after modifications** - Persist to disk for session continuity

## PR/Commit Guidelines

- Format: `type: brief description`
- Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`
- Keep commits atomic and focused
- Run `cargo fmt` and `cargo clippy` before committing

## Before Finalizing Changes

1. `cargo fmt` - Format code
2. `cargo clippy --all-features` - Check for lints
3. `cargo test` - Run tests
4. `cargo build --release` - Verify release build

## Debugging

Enable detailed logging:

```bash
RUST_LOG=codetether=debug codetether tui
RUST_LOG=codetether::session=trace codetether run "test"
```

## Performance Notes

- Startup target: <15ms
- Memory target: <20MB idle
- Use `Arc` for shared provider state
- Prefer streaming responses for large outputs
