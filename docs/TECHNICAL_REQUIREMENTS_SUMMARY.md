# CodeTether Agent - Technical Requirements Summary

## Overview

This document provides a structured summary of technical requirements extracted from the PRD document (`docs/PRD.md`) and implementation files. It covers the three main features: Autonomous PRD-Driven Development (Ralph), LSP Client Integration, and Recursive Language Model Processing.

---

## 1. Autonomous PRD-Driven Development (Ralph)

### 1.1 Core Architecture

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Language** | Rust (Edition 2024, MSRV 1.85) | High-performance, memory-safe implementation |
| **Async Runtime** | Tokio (full features) | Concurrent execution and I/O handling |
| **Serialization** | serde + serde_json | PRD parsing and state persistence |
| **Process Management** | std::process::Command | Quality gate execution |

### 1.2 PRD Structure Requirements

```rust
// Key data structures from src/ralph/types.rs
pub struct UserStory {
    pub id: String,                    // Unique identifier (e.g., "US-001")
    pub title: String,                 // Short title
    pub description: String,           // Full description
    pub acceptance_criteria: Vec<String>,
    pub passes: bool,                  // Quality gate status
    pub priority: u8,                  // 1=highest
    pub depends_on: Vec<String>,       // Dependency IDs
    pub complexity: u8,                // 1-5 scale
}

pub struct Prd {
    pub project: String,
    pub feature: String,
    pub branch_name: String,
    pub version: String,
    pub user_stories: Vec<UserStory>,
    pub technical_requirements: Vec<String>,
    pub quality_checks: QualityChecks,
}
```

### 1.3 Quality Gates (Mandatory)

All implementations MUST pass these quality checks:

| Check | Command | Purpose |
|-------|---------|---------|
| Type Checking | `cargo check` | Compile-time type safety |
| Linting | `cargo clippy -- -D warnings` | Code quality and style |
| Testing | `cargo test` | Functional correctness |
| Build | `cargo build --release` | Production-ready binary |

### 1.4 Execution Modes

| Mode | Description | Use Case |
|------|-------------|----------|
| **Sequential** | Stories executed one at a time | Dependencies require ordering |
| **Parallel** | Stories grouped by dependency stages | Independent stories run concurrently |

### 1.5 Parallel Execution Requirements

- **Max Concurrent Stories**: Configurable (default: 3)
- **Worktree Isolation**: Git worktrees for parallel story isolation
- **Semaphore-based Concurrency**: tokio::sync::Semaphore
- **Merge Strategy**: Automatic worktree merge on success

### 1.6 Memory Persistence

| Mechanism | Storage | Purpose |
|-----------|---------|---------|
| Git History | `.git/` | Code changes and versioning |
| Progress Log | `progress.txt` | Learnings and context across iterations |
| PRD State | `prd.json` | Story completion tracking |

### 1.7 Configuration

```rust
pub struct RalphConfig {
    pub prd_path: String,              // Default: "prd.json"
    pub max_iterations: usize,         // Default: 10
    pub progress_path: String,         // Default: "progress.txt"
    pub auto_commit: bool,             // Default: false
    pub quality_checks_enabled: bool,  // Default: true
    pub parallel_enabled: bool,        // Default: true
    pub max_concurrent_stories: usize, // Default: 3
    pub worktree_enabled: bool,        // Default: true
}
```

### 1.8 Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Story Implementation Pass Rate | ≥95% | Quality check results |
| Average Time Per Story | <5 minutes | Timestamp analysis |
| Token Efficiency | 3x reduction vs manual | Token usage comparison |

---

## 2. Language Server Protocol (LSP) Client Integration

### 2.1 Transport Layer Requirements

| Requirement | Specification | Implementation |
|-------------|---------------|----------------|
| Transport | stdio | tokio::process |
| Protocol | JSON-RPC 2.0 | jsonrpc-core crate |
| Message Framing | Content-Length headers | Custom parser |
| Encoding | UTF-8 | Built-in Rust support |

### 2.2 LSP Lifecycle

```
Initialize Request → Initialize Response → Initialized Notification
                        ↓
            [Document Synchronization]
                        ↓
            Shutdown Request → Exit Notification
```

### 2.3 Core LSP Operations

| Operation | Method | Parameters | Response Type |
|-----------|--------|------------|---------------|
| Go to Definition | `textDocument/definition` | file_path, line, column | Location/LocationLink |
| Find References | `textDocument/references` | file_path, line, column | Location[] |
| Hover | `textDocument/hover` | file_path, line, column | Hover/MarkupContent |
| Completion | `textDocument/completion` | file_path, line, column | CompletionList |
| Document Symbol | `textDocument/documentSymbol` | file_path | SymbolInformation[] |
| Workspace Symbol | `workspace/symbol` | query | SymbolInformation[] |

### 2.4 Document Synchronization

| Notification | Purpose |
|--------------|---------|
| `textDocument/didOpen` | Notify server of opened document |
| `textDocument/didChange` | Incremental or full document updates |
| `textDocument/didClose` | Notify server of closed document |

### 2.5 Server Management

| Feature | Requirement |
|---------|-------------|
| Multi-server Support | Concurrent connections to different language servers |
| Server Discovery | Configuration-based server executable paths |
| Health Monitoring | Automatic restart on crash |
| Graceful Shutdown | Proper cleanup sequence |

### 2.6 Supported Language Servers

| Language | Server | Status |
|----------|--------|--------|
| Rust | rust-analyzer | Verified |
| TypeScript | typescript-language-server | Verified |
| Python | pylsp | Target |
| Go | gopls | Target |
| C/C++ | clangd | Target |

### 2.7 Performance Requirements

| Metric | Target |
|--------|--------|
| Request-Response Latency | <100ms p95 |
| Server Uptime (with auto-restart) | 99.9% |

### 2.8 Current Implementation Status

**Note**: The LSP tool (`src/tool/lsp.rs`) currently has a **placeholder implementation**. Full LSP client integration is documented in `prd.json` with 10 user stories (US-001 through US-010), all marked as passing.

---

## 3. Recursive Language Model (RLM) Processing

### 3.1 Core Concept

RLM handles contexts exceeding model window limits through:
1. **Intelligent Chunking**: Content split at semantic boundaries
2. **REPL Environment**: Internal environment for content exploration
3. **Sub-LM Queries**: Recursive semantic analysis across chunks
4. **Result Synthesis**: Coherent answers from multiple analyses

### 3.2 Content Type Detection

| Content Type | Detection Pattern | Processing Focus |
|--------------|-------------------|------------------|
| Code | Function/class definitions, imports | Structure, dependencies, logic flow |
| Logs | Timestamps, log levels | Errors, warnings, event sequences |
| Conversation | [User]:, [Assistant]: markers | Decisions, tool calls, state |
| Documents | Markdown headers, prose | Topics, facts, actionable items |

### 3.3 Chunking Strategy

```rust
pub struct Chunk {
    pub content: String,
    pub chunk_type: ChunkType,     // Code, Text, ToolOutput, Conversation
    pub start_line: usize,
    pub end_line: usize,
    pub tokens: usize,
    pub priority: u8,              // Higher = more important
}

pub struct ChunkOptions {
    pub max_chunk_tokens: usize,   // Default: 4000
    pub preserve_recent: usize,    // Default: 100 lines
}
```

### 3.4 REPL Environment

| Function | Description |
|----------|-------------|
| `head(n)` | First n lines |
| `tail(n)` | Last n lines |
| `grep("pattern")` | Search with regex |
| `count("pattern")` | Count matches |
| `slice(start, end)` | Character slice |
| `chunks(n)` | Split into n chunks |
| `llm_query("q")` | Sub-LM semantic query |
| `FINAL("answer")` | Return final answer |

### 3.5 RLM Configuration

```rust
pub struct RlmConfig {
    pub mode: String,              // "auto", "off", "always"
    pub threshold: f64,            // 0.0-1.0 (default: 0.35)
    pub max_iterations: usize,     // Default: 15
    pub max_subcalls: usize,       // Default: 50
    pub runtime: String,           // "rust", "bun", "python"
    pub root_model: Option<String>,
    pub subcall_model: Option<String>,
}
```

### 3.6 Routing Logic

```rust
pub struct RoutingContext {
    pub tool_id: String,           // Tool producing output
    pub session_id: String,
    pub model_context_limit: usize,
    pub current_context_tokens: Option<usize>,
}
```

**Routing Decision Tree:**
1. If `mode == "off"` → Don't route
2. If `mode == "always"` and tool eligible → Route
3. If `mode == "auto"`:
   - Check if output exceeds threshold (default: 35% of context window)
   - Check if adding output would cause overflow (>80% of context)

### 3.7 Eligible Tools for RLM

- `read` - File contents
- `glob` - File listings
- `grep` - Search results
- `bash` - Command output
- `search` - Code search results

### 3.8 Performance Requirements

| Metric | Target |
|--------|--------|
| Maximum Context Size | 10MB+ |
| Chunk Processing Throughput | >100 chunks/sec |
| Answer Accuracy | ≥90% |

### 3.9 RLM Pool (Connection Pooling)

| Feature | Requirement |
|---------|-------------|
| Pool Size | Configurable idle agents |
| Acquire/Release | Efficient agent reuse |
| Cleanup | Automatic stale connection removal |
| Metrics | Pool utilization tracking |

---

## 4. System Architecture

### 4.1 Project Structure

```
src/
├── a2a/           # Agent-to-Agent protocol
├── agent/         # Agent implementation
├── cli/           # Command-line interface
├── config/        # Configuration management
├── mcp/           # Model Context Protocol
├── provider/      # AI provider integrations
├── ralph/         # Autonomous PRD-driven development
├── rlm/           # Recursive Language Model
├── secrets/       # Vault integration
├── server/        # HTTP server mode
├── session/       # Session management
├── swarm/         # Agent swarm orchestration
├── tool/          # Tool system (24+ tools)
├── tui/           # Terminal UI
└── worktree/      # Git worktree management
```

### 4.2 Key Dependencies

| Category | Crates |
|----------|--------|
| Async Runtime | tokio |
| HTTP/Web | axum, reqwest, tower |
| Serialization | serde, serde_json, toml |
| AI Providers | async-openai |
| LSP | lsp-types, jsonrpc-core |
| TUI | ratatui, crossterm |
| Secrets | vaultrs |
| Utilities | anyhow, thiserror, tracing, regex, glob |

### 4.3 Tool Registry

The system provides 24+ built-in tools:

| Tool | Purpose |
|------|---------|
| read/write/list/glob | File operations |
| grep/search/codesearch | Code search |
| edit/multiedit/patch | Code editing |
| bash | Shell execution |
| lsp | LSP operations |
| webfetch/websearch | Web access |
| todo/task/plan | Task management |
| ralph | Autonomous execution |
| rlm | Recursive processing |
| prd | PRD management |
| skill | Skill loading |
| question | User interaction |
| batch | Parallel tool execution |

---

## 5. User Personas & Use Cases

### 5.1 Senior Software Engineer (Alex)
- **Needs**: Automate repetitive tasks, navigate large codebases
- **Features**: Ralph for autonomous implementation, LSP for code intelligence, RLM for codebase queries

### 5.2 Tech Lead (Maya)
- **Needs**: Standardize practices, accelerate delivery
- **Features**: PRD-driven development, quality gates, parallel execution

### 5.3 Indie Developer (Jordan)
- **Needs**: Ship fast as solo dev, maintain quality
- **Features**: Autonomous implementation as "virtual teammate", multi-language support

### 5.4 AI Research Engineer (Dr. Chen)
- **Needs**: Analyze large codebases, process logs
- **Features**: RLM for arbitrarily large contexts, swarm mode for parallel analysis

---

## 6. Scope Boundaries

### 6.1 In Scope

- A2A protocol implementation
- 24+ built-in tools
- LSP client (stdio transport)
- RLM processing with chunking
- Session management with git-aware storage
- Interactive TUI and HTTP server mode
- HashiCorp Vault integration
- Multiple AI provider support

### 6.2 Out of Scope

| Feature | Status |
|---------|--------|
| LSP Server Implementation | Not implemented |
| GUI/IDE Integration | TUI and CLI only |
| Collaborative Editing | Not implemented |
| Version Control Operations | Limited to change tracking |
| Deployment Automation | Not implemented |
| Natural Language PRD Creation | Structured JSON only |
| Mobile Applications | Desktop/server only |
| Offline Operation | Requires internet |
| Custom Model Training | Not implemented |

### 6.3 External Dependencies

| Component | Requirement |
|-----------|-------------|
| Language Servers | Must be installed separately |
| HashiCorp Vault | External server required |
| A2A Server | CodeTether is a worker, not server |
| MCP Server | Client only |

---

## 7. Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Binary Startup | <15ms | `time codetether --help` |
| Memory (idle) | <20MB | `ps` memory reporting |
| Memory (swarm, 10 agents) | <60MB | Peak RSS |
| Sub-agent Spawn | <2ms | Instrumented latency |
| LSP Latency | <100ms p95 | Request-response |
| RLM Throughput | >100 chunks/sec | Benchmark corpus |

---

## 8. Quality Assurance

### 8.1 Definition of Done

- [ ] All acceptance criteria met
- [ ] Code passes all quality gates
- [ ] Documentation updated
- [ ] Changes committed to git
- [ ] PRD updated to mark story passing
- [ ] No regression in existing functionality

### 8.2 Test Coverage Targets

| Metric | Target |
|--------|--------|
| Documentation Coverage | ≥80% |
| Test Coverage | ≥70% |

---

## 9. References

- [LSP Specification 3.17](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
- [A2A Protocol](https://github.com/google/A2A)
- [JSON-RPC 2.0](https://www.jsonrpc.org/specification)
- [Recursive Language Model Paper](https://arxiv.org/abs/2305.14976)

---

*Document generated from PRD review - February 2025*
