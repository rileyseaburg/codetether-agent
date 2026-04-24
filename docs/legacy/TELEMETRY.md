# CodeTether Telemetry System Documentation

## Overview

The telemetry system in CodeTether tracks token usage, costs, tool executions, file changes, and other metrics for monitoring agent performance and providing audit trails. The system is located in `src/telemetry/mod.rs`.

## Architecture

The telemetry system is organized into four main components:

1. **Tool Execution Tracking** - Records all tool executions with detailed metadata
2. **Token Usage Tracking** - Monitors LLM token consumption by model and operation
3. **Cost Estimation** - Calculates approximate costs based on model pricing
4. **Persistence** - Saves telemetry data to disk for historical analysis

---

## Public Modules and Types

### 1. Tool Execution Tracking

#### Core Types

**`ToolExecId`** (type alias)
```rust
pub type ToolExecId = u64;
```
Unique identifier for a tool execution.

**`ToolExecution`** (struct)
Records a single tool execution with comprehensive metadata:
- `id: ToolExecId` - Unique execution ID
- `tool_name: String` - Tool name/identifier
- `input: serde_json::Value` - Full input arguments (JSON)
- `output: Option<String>` - Output result
- `success: bool` - Whether execution succeeded
- `error: Option<String>` - Error message if failed
- `started_at: u64` - Execution start time (Unix timestamp ms)
- `duration_ms: u64` - Duration in milliseconds
- `files_affected: Vec<FileChange>` - Files affected by this tool
- `tokens: Option<TokenCounts>` - Token usage for this execution
- `parent_id: Option<ToolExecId>` - Parent execution ID (for nested/sub-agent calls)
- `session_id: Option<String>` - Session or agent ID
- `model: Option<String>` - Model used (if applicable)
- `metadata: HashMap<String, serde_json::Value>` - Additional metadata

**`ToolExecution` Methods:**
- `start(tool_name, input)` - Create a new execution record (call when starting)
- `complete_success(output, duration)` - Complete with success
- `complete_error(error, duration)` - Complete with error
- `add_file_change(change)` - Add a file change
- `with_parent(parent_id)` - Set parent execution ID
- `with_session(session_id)` - Set session ID
- `with_model(model)` - Set model

**`FileChangeType`** (enum)
```rust
pub enum FileChangeType {
    Create,
    Modify,
    Delete,
    Rename,
    Read,
}
```

**`FileChange`** (struct)
Records a single file change:
- `change_type: FileChangeType` - Type of change
- `path: String` - File path (relative to workspace)
- `old_path: Option<String>` - Old path (for renames)
- `lines_affected: Option<(u32, u32)>` - Lines affected (start, end) - 1-indexed
- `before: Option<String>` - Content before change
- `after: Option<String>` - Content after change
- `diff: Option<String>` - Unified diff (if available)
- `size_before: Option<u64>` - Size in bytes before
- `size_after: Option<u64>` - Size in bytes after
- `timestamp: u64` - Timestamp (Unix ms)

**`FileChange` Methods:**
- `create(path, content)` - Create a file creation record
- `modify(path, before, after, lines)` - Create a modification record
- `modify_with_diff(path, before, after, diff, lines)` - Create with diff
- `delete(path, content)` - Create a deletion record
- `read(path, lines)` - Create a read record (for audit trail)
- `rename(old_path, new_path)` - Create a rename record
- `summary()` - Get a short summary of the change

**`ToolExecutionTracker`** (struct)
Thread-safe tool execution tracker with indexing:
- `new()` - Create with default capacity (1000)
- `with_capacity(max)` - Create with specified capacity
- `record(execution)` - Record a completed execution
- `get(id)` - Get execution by ID
- `get_by_tool(tool_name)` - Get all executions for a tool
- `get_by_file(path)` - Get executions that affected a file
- `recent(count)` - Get recent executions (last N)
- `all_file_changes()` - Get all file changes across executions
- `stats()` - Get statistics

**`ToolExecutionStats`** (struct)
Statistics about tool executions:
- `total_executions: u64`
- `successful_executions: u64`
- `failed_executions: u64`
- `total_duration_ms: u64`
- `avg_duration_ms: u64`
- `executions_by_tool: HashMap<String, u64>`
- `unique_files_affected: u64`
- `summary()` - Format as summary string

**Global Static:**
```rust
pub static TOOL_EXECUTIONS: Lazy<ToolExecutionTracker>
```

---

### 2. Token Usage Tracking

#### Core Types

**`TokenCounts`** (struct)
```rust
pub struct TokenCounts {
    pub input: u64,
    pub output: u64,
}
```
- `new(input, output)` - Create new token counts
- `total()` - Total tokens (input + output)
- `add(other)` - Add another TokenCounts to this one
- Implements `Display` for formatting

**`TokenStats`** (struct)
```rust
pub struct TokenStats {
    pub avg_input: f64,
    pub avg_output: f64,
    pub avg_total: f64,
    pub max_input: u64,
    pub max_output: u64,
    pub max_total: u64,
}
```

**`TokenUsageTracker`** (struct)
Thread-safe token usage tracker for a single model:
- `new(name)` - Create tracker for named model/operation
- `record(input, output)` - Record token usage
- `record_counts(counts)` - Record with TokenCounts
- `totals()` - Get current totals (fast, atomic)
- `request_count()` - Get request count
- `snapshot()` - Get immutable snapshot

**`TokenUsageSnapshot`** (struct)
Immutable snapshot of token usage state:
- `name: String`
- `totals: TokenCounts`
- `request_count: u64`
- `stats: TokenStats`
- `summary()` - Display summary
- `detailed()` - Display detailed stats

**`TokenUsageRegistry`** (struct)
Tracks usage by model and operation type:
- `new()` - Create new registry
- `record_model_usage(model, input, output)` - Record for a model
- `record_operation_usage(operation, input, output)` - Record for operation
- `get_model_tracker(model)` - Get tracker for model
- `get_operation_tracker(operation)` - Get tracker for operation
- `global_tracker()` - Get global tracker
- `model_snapshots()` - Get all model snapshots
- `operation_snapshots()` - Get all operation snapshots
- `global_snapshot()` - Get global snapshot

**Global Static:**
```rust
pub static TOKEN_USAGE: Lazy<TokenUsageRegistry>
```

---

### 3. Cost Estimation

**`CostEstimate`** (struct)
```rust
pub struct CostEstimate {
    pub input_cost: f64,
    pub output_cost: f64,
    pub total_cost: f64,
}
```
- `from_tokens(counts, input_cost_per_million, output_cost_per_million)` - Calculate cost
- `format_currency()` - Format as currency (e.g., "$0.0123")
- `format_smart()` - Format with appropriate precision

**Supported Model Pricing** (in `src/tui/token_display.rs`):
- GPT-4o-mini: $0.15 / $0.60 per million
- GPT-4o: $2.50 / $10.00 per million
- GPT-4-turbo: $10.00 / $30.00 per million
- GPT-4: $30.00 / $60.00 per million
- Claude-3.5-sonnet: $3.00 / $15.00 per million
- Claude-3.5-haiku: $0.80 / $4.00 per million
- Claude-3-opus: $15.00 / $75.00 per million
- Gemini-2.0-flash: $0.075 / $0.30 per million
- Gemini-1.5-flash: $0.075 / $0.30 per million
- Gemini-1.5-pro: $1.25 / $5.00 per million
- GLM-4: ~$0.50 / $0.50 per million
- K1.5: $8.00 / $8.00 per million
- K1.6: $6.00 / $6.00 per million

---

### 4. Context Limit Tracking

**`ContextLimit`** (struct)
```rust
pub struct ContextLimit {
    pub current: u64,
    pub limit: u64,
    pub percentage: f64,
}
```
- `new(current, limit)` - Create new context limit info
- `warning_level()` - Get warning level ("OK", "LOW", "MEDIUM", "HIGH", "CRITICAL")
- `warning_color()` - Get ratatui Color for warning level

**Supported Context Limits** (in `src/tui/token_display.rs`):
- GPT-4 series: 128,000 tokens
- Claude-3.5 series: 200,000 tokens
- Gemini-2.0-flash: 1,000,000 tokens
- Gemini-1.5-pro: 2,000,000 tokens
- K1.5/K1.6: 200,000 tokens

---

### 5. Persistence

**`TelemetryData`** (struct)
Persistent telemetry data:
- `executions: Vec<ToolExecution>` - Recent executions (last 500)
- `stats: TelemetryStats` - Aggregated statistics
- `last_updated: u64` - Last update timestamp

**`TelemetryData` Methods:**
- `default_path()` - Get default file path (`~/.local/share/codetether/telemetry.json`)
- `load()` - Load from default path
- `load_from(path)` - Load from specific path
- `save()` - Save to default path
- `save_to(path)` - Save to specific path
- `record_execution(exec)` - Add execution and update stats
- `recent(count)` - Get recent executions
- `by_tool(tool_name)` - Get executions by tool
- `by_file(path)` - Get executions affecting a file
- `all_file_changes()` - Get all file changes
- `summary()` - Format as summary

**`TelemetryStats`** (struct)
Aggregated statistics:
- `total_executions: u64`
- `successful_executions: u64`
- `failed_executions: u64`
- `total_duration_ms: u64`
- `total_input_tokens: u64`
- `total_output_tokens: u64`
- `total_requests: u64`
- `executions_by_tool: HashMap<String, u64>`
- `files_modified: HashMap<String, u64>`

**Global Static:**
```rust
pub static PERSISTENT_TELEMETRY: Lazy<RwLock<TelemetryData>>
```

**Helper Functions:**
- `record_persistent(exec)` - Record to persistent telemetry
- `get_persistent_stats()` - Get snapshot of persistent telemetry

---

## Configuration

### Data Storage Location

Telemetry data is stored at:
- **Default path**: `~/.local/share/codetether/codetether/telemetry.json` (Linux)
- **macOS**: `~/Library/Application Support/com.codetether.codetether/telemetry.json`
- **Windows**: `%APPDATA%\codetether\codetether\telemetry.json`
- **Fallback**: `.codetether/telemetry.json` (in current directory)

### Current Configuration Options

**Note**: The telemetry system currently does NOT have explicit enable/disable configuration. It is always active when the application runs.

The system operates with the following defaults:
- **In-memory execution limit**: 1000 (configurable via `ToolExecutionTracker::with_capacity()`)
- **Persistent execution limit**: 500 (hardcoded in `TelemetryData::record_execution()`)
- **Persistence**: Automatic on each execution record

### Usage in Code

#### Recording Tool Executions

```rust
use crate::telemetry::{TOOL_EXECUTIONS, ToolExecution, record_persistent};
use std::time::Instant;

async fn execute_tool(args: Value) -> Result<ToolResult> {
    let start = Instant::now();
    
    // Start tracking
    let exec = ToolExecution::start("tool_name", args.clone());
    
    // ... do work ...
    
    // Complete tracking
    let exec = exec.complete_success(output, start.elapsed());
    
    // Record to in-memory tracker
    TOOL_EXECUTIONS.record(exec.clone());
    
    // Record to persistent storage
    record_persistent(exec);
    
    Ok(result)
}
```

#### Recording File Changes

```rust
use crate::telemetry::FileChange;

// File creation
let change = FileChange::create("src/main.rs", content);

// File modification
let change = FileChange::modify("src/main.rs", old_content, new_content, Some((10, 20)));

// File deletion
let change = FileChange::delete("src/main.rs", content);

// Add to execution
exec.add_file_change(change);
```

#### Recording Token Usage

```rust
use crate::telemetry::TOKEN_USAGE;

// Record model usage
TOKEN_USAGE.record_model_usage("gpt-4o", input_tokens, output_tokens);

// Record operation usage
TOKEN_USAGE.record_operation_usage("code_review", input_tokens, output_tokens);

// Get snapshots
let global = TOKEN_USAGE.global_snapshot();
let models = TOKEN_USAGE.model_snapshots();
```

#### Calculating Costs

```rust
use crate::telemetry::{CostEstimate, TokenCounts};

let counts = TokenCounts::new(1000, 500);
let cost = CostEstimate::from_tokens(&counts, 2.50, 10.00);
println!("Cost: {}", cost.format_currency()); // "$0.0125"
```

---

## Integration Points

The telemetry system is integrated at the following points:

1. **Session Module** (`src/session/mod.rs`):
   - Records token usage for each LLM completion
   - Uses `TOKEN_USAGE.record_model_usage()`

2. **Tool Implementations**:
   - `src/tool/bash.rs` - Records bash command executions
   - `src/tool/file.rs` - Records file reads with `FileChange::read()`
   - `src/tool/confirm_edit.rs` - Records file modifications with `FileChange::modify()`

3. **TUI Display** (`src/tui/token_display.rs`):
   - Displays token usage and costs in status bar
   - Shows context limit warnings
   - Provides detailed token usage breakdown

4. **Main Entry** (`src/main.rs`):
   - Accesses `TOKEN_USAGE`, `TOOL_EXECUTIONS`, and `get_persistent_stats()`

---

## Data Collected

### Tool Executions
- Tool name and input arguments
- Success/failure status and errors
- Execution duration
- Files affected (create, modify, delete, rename, read)
- Token usage per execution
- Session and model association
- Parent-child relationships for nested calls

### Token Usage
- Input and output token counts
- Per-model tracking
- Per-operation-type tracking
- Global aggregation
- Request counts and averages
- Maximum usage per session

### Cost Metrics
- Estimated costs per model
- Session total costs
- Based on approximate pricing (not exact billing)

### File Changes
- File paths and change types
- Line ranges affected
- Before/after content (for modifications)
- Unified diffs (when available)
- File sizes

---

## Privacy Considerations

- **Local Storage**: All telemetry is stored locally on the user's machine
- **No Remote Transmission**: Data is not sent to any external servers
- **Content Capture**: File content may be stored in change records
- **Argument Logging**: Tool input arguments are stored as JSON
- **User Control**: Users can delete `telemetry.json` to clear history

---

## Future Enhancements

Potential configuration options that could be added:

```toml
[telemetry]
enabled = true                    # Enable/disable telemetry
persist = true                    # Save to disk
max_executions = 500              # In-memory limit
include_file_content = true       # Store file content in changes
include_tool_args = true          # Store tool input arguments
anonymize_paths = false           # Hash file paths
```
