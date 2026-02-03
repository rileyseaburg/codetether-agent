# Tool Registration Design: batch.rs and invalid.rs

## Executive Summary

This document defines the registration design for `BatchTool` and `InvalidTool`, determining their availability, integration patterns, and lifecycle management within the tool registry system.

---

## 1. Tool Purposes

### 1.1 BatchTool (`src/tool/batch.rs`)

**Purpose**: Execute multiple tool calls in parallel within a single invocation.

**Key Characteristics**:
- **Self-referential**: Requires access to the ToolRegistry to look up and invoke other tools
- **Circular dependency risk**: The registry contains the batch tool, but batch tool needs registry access
- **Recursive protection**: Prevents calling `batch` from within `batch` to avoid infinite loops
- **Parallel execution**: Uses `futures::future::join_all` for concurrent tool execution

**Use Cases**:
- Run multiple independent file reads simultaneously
- Execute parallel searches across different patterns
- Batch file operations (read, edit, write) in one call
- Reduce LLM round-trips for multi-step operations

### 1.2 InvalidTool (`src/tool/batch.rs`)

**Purpose**: Provide graceful error handling when a model attempts to call a non-existent tool.

**Key Characteristics**:
- **Fallback mechanism**: Returns helpful error messages with suggestions
- **Fuzzy matching**: Uses Levenshtein distance to suggest similar tool names
- **Context-aware**: Can be pre-configured with available tool list for better suggestions
- **Always succeeds structurally**: Returns ToolResult with error message (not a Rust Err)

**Use Cases**:
- Handle typos in tool names (e.g., "red" instead of "read")
- Provide discoverability when models hallucinate tool names
- Register as a catch-all for unknown tool IDs
- Improve debugging with detailed error context

---

## 2. Registration Design

### 2.1 Availability Matrix

| Tool | Always Available | Conditional | Special Registration |
|------|------------------|-------------|---------------------|
| **BatchTool** | ✅ Yes | No | Requires two-phase init |
| **InvalidTool** | ✅ Yes | No | Standard registration |

### 2.2 BatchTool Registration Pattern

**Problem**: BatchTool needs a reference to ToolRegistry, but ToolRegistry owns BatchTool (circular dependency).

**Solution**: Two-phase initialization with `Weak` reference

```rust
// Phase 1: Create registry with all standard tools
let mut registry = ToolRegistry::with_defaults();

// Phase 2: Create batch tool (no registry reference yet)
let batch_tool = Arc::new(batch::BatchTool::new());
registry.register(batch_tool.clone());

// Phase 3: Wrap in Arc
let registry = Arc::new(registry);

// Phase 4: Give batch tool weak reference to registry
batch_tool.set_registry(Arc::downgrade(&registry));
```

**Implementation** (from `mod.rs`):
```rust
pub fn with_defaults_arc() -> Arc<Self> {
    let mut registry = Self::with_defaults();
    
    // Create batch tool without registry reference
    let batch_tool = Arc::new(batch::BatchTool::new());
    registry.register(batch_tool.clone());
    
    // Wrap registry in Arc
    let registry = Arc::new(registry);
    
    // Now give batch tool a weak reference to the registry
    batch_tool.set_registry(Arc::downgrade(&registry));
    
    registry
}
```

**Why Weak Reference?**
- Prevents reference cycles (Arc<->Arc would leak memory)
- Allows graceful degradation if registry is dropped
- Runtime check via `weak.upgrade()` returns `None` if registry gone

### 2.3 InvalidTool Registration Pattern

**Standard Registration** (simple Arc::new):
```rust
registry.register(Arc::new(invalid::InvalidTool::new()));
```

**Current Implementation** (from `mod.rs`):
```rust
// Register the invalid tool handler for graceful error handling
registry.register(Arc::new(invalid::InvalidTool::new()));
```

**Dual Usage Pattern**:

InvalidTool serves two purposes in the system:

1. **Registered Tool**: Available as `invalid` tool for explicit error reporting
2. **Runtime Fallback**: Instantiated dynamically when unknown tools are called

**Runtime Usage** (from `src/agent/mod.rs`):
```rust
None => {
    // Use the invalid tool handler for better error messages
    let available_tools = self.tools.list().iter().map(|s| s.to_string()).collect();
    let invalid_tool = crate::tool::invalid::InvalidTool::with_context(name.to_string(), available_tools);
    let args = serde_json::json!({
        "requested_tool": name,
        "args": serde_json::from_str::<serde_json::Value>(arguments).unwrap_or(serde_json::json!({}))
    });
    match invalid_tool.execute(args).await {
        Ok(result) => result,
        Err(e) => ToolResult {
            output: format!("Unknown tool: {}. Error: {}", name, e),
            success: false,
            metadata: HashMap::new(),
        },
    }
}
```

This pattern is used in:
- `Agent::execute_tool()` - For direct tool execution
- `Agent::handle_message()` - For swarm message handling  
- `BatchTool::execute()` - For unknown tools within batch calls

---

## 3. Integration with Existing Tool System

### 3.1 ToolRegistry Factory Methods

| Method | BatchTool | InvalidTool | Use Case |
|--------|-----------|-------------|----------|
| `new()` | ❌ No | ❌ No | Empty registry |
| `with_defaults()` | ❌ No | ✅ Yes | Basic registry (non-Arc) |
| `with_provider()` | ❌ No | ✅ Yes | Registry with provider |
| `with_defaults_arc()` | ✅ Yes | ✅ Yes | **Standard usage** |
| `with_provider_arc()` | ✅ Yes | ✅ Yes | With provider + batch |

### 3.2 Recommended Usage Pattern

**For most use cases**:
```rust
let registry = ToolRegistry::with_defaults_arc();
```

This provides:
- All standard tools (file, edit, bash, etc.)
- BatchTool with proper circular dependency resolution
- InvalidTool for explicit error handling

### 3.3 Current Usage in Codebase

**Primary Usage Location** (`src/swarm/executor.rs:215`):
```rust
// Create shared tool registry with provider for ralph and batch tool
let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
let tool_definitions = tool_registry.definitions();
```

The swarm executor uses `with_provider_arc()` to:
1. Provide all tools to sub-agents
2. Enable RalphTool with provider access for autonomous execution
3. Include BatchTool for parallel operations
4. Include InvalidTool for error handling

### 3.4 Tool Definition Exposure

Both tools are exposed to LLMs via `ToolRegistry::definitions()`:

```rust
pub fn definitions(&self) -> Vec<crate::provider::ToolDefinition> {
    self.tools
        .values()
        .map(|t| crate::provider::ToolDefinition {
            name: t.id().to_string(),
            description: t.description().to_string(),
            parameters: t.parameters(),
        })
        .collect()
}
```

**Implications**:
- LLMs CAN see and call `batch` tool
- LLMs CAN see and call `invalid` tool (though unusual)
- Both appear in tool definitions sent to providers

---

## 4. Security & Safety Considerations

### 4.1 BatchTool Safety

| Concern | Mitigation |
|---------|------------|
| Recursive batch calls | Explicitly blocked in code |
| Registry unavailable | Returns error: "Registry not initialized" |
| Registry dropped | Returns error: "Registry no longer available" |
| Unknown tools in batch | Returns error per call, continues others |

### 4.2 InvalidTool Safety

| Concern | Mitigation |
|---------|------------|
| Information leakage | Only exposes tool names (already known to LLM) |
| DoS via suggestions | Limited to 3 similar tools max |
| Empty tool list | Graceful fallback message |

---

## 5. Future Enhancements

### 5.1 Potential Improvements

1. **Automatic InvalidTool Resolution**
   - Modify `ToolRegistry::get()` to return InvalidTool for unknown IDs
   - Would require changing return type or internal handling

2. **BatchTool Size Limits**
   - Add maximum number of calls per batch
   - Prevent resource exhaustion from huge batches

3. **Conditional Registration**
   - Feature flags to disable batch/invalid in certain contexts
   - Environment-based tool availability

4. **BatchTool Timeouts**
   - Per-call timeout in batch execution
   - Overall batch timeout

### 5.2 Alternative Registration Patterns

**Lazy Initialization** (alternative to two-phase):
```rust
pub struct BatchTool {
    registry: OnceLock<Weak<ToolRegistry>>,
}
```

**Registry Callback** (alternative architecture):
```rust
pub trait RegistryAccessor {
    fn with_registry<F, R>(&self, f: F) -> R 
    where F: FnOnce(&ToolRegistry) -> R;
}
```

---

## 6. Summary

| Aspect | BatchTool | InvalidTool |
|--------|-----------|-------------|
| **Availability** | Always (via `with_defaults_arc`) | Always (all factory methods) |
| **Registration** | Two-phase with Weak reference | Standard Arc::new |
| **Special Needs** | Circular dependency resolution | None |
| **LLM Visible** | Yes | Yes |
| **Primary Use** | Parallel tool execution | Graceful error handling |
| **Integration** | Complex (self-referential) | Simple (standalone) |

**Key Design Decisions**:
1. ✅ BatchTool uses `Weak<ToolRegistry>` to break circular dependency
2. ✅ Two-phase initialization in `with_defaults_arc()` and `with_provider_arc()`
3. ✅ InvalidTool registered as standard tool (not automatic fallback)
4. ✅ Both tools included in default Arc-based registry constructors
5. ✅ BatchTool prevents recursive calls to avoid infinite loops
