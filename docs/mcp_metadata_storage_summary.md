# MCP Metadata Storage System - Quick Summary

## Problem

The current `McpServer` in `src/mcp/server.rs` has a separation between metadata (`McpTool`, `McpResource`) and handlers:

- `McpTool`/`McpResource` structs are created in `register_tool`/`register_resource` but discarded
- Only handlers are stored in `HashMap<String, Handler>`
- `handle_list_tools` hardcodes tool definitions instead of using registered data

## Solution

Create unified metadata structures that combine definition + handler + metadata.

## Key Data Structures

```rust
// New metadata types that combine definition + handler
pub struct ToolMetadata {
    pub definition: McpTool,           // Protocol definition
    pub handler: McpToolHandler,       // Execution handler
    pub registered_at: Instant,        // Audit info
    pub custom: HashMap<String, Value>, // Extensions
}

pub struct ResourceMetadata {
    pub definition: McpResource,
    pub handler: McpResourceHandler,
    pub registered_at: Instant,
    pub custom: HashMap<String, Value>,
}

pub struct PromptMetadata {
    pub definition: McpPrompt,
    pub handler: McpPromptHandler,
    pub registered_at: Instant,
    pub custom: HashMap<String, Value>,
}

// Thread-safe storage wrapper
pub struct MetadataMap<K, V> {
    inner: RwLock<HashMap<K, V>>,
}
```

## Updated Server Structure

```rust
pub struct McpServer {
    transport: Arc<dyn Transport>,
    tools: MetadataMap<String, ToolMetadata>,      // Changed from HashMap<String, Handler>
    resources: MetadataMap<String, ResourceMetadata>,
    prompts: MetadataMap<String, PromptMetadata>,
    initialized: RwLock<bool>,
    server_info: ServerInfo,
}
```

## Type Signatures

```rust
pub type McpToolHandler = Arc<
    dyn Fn(Value) -> Result<CallToolResult> + Send + Sync
>;

pub type McpResourceHandler = Arc<
    dyn Fn(String) -> Result<ReadResourceResult> + Send + Sync
>;

pub type McpPromptHandler = Arc<
    dyn Fn(Value) -> Result<GetPromptResult> + Send + Sync
>;
```

## Keying Strategy

| Entity | Key Type | Key Source |
|--------|----------|------------|
| Tool | `String` | `McpTool.name` |
| Resource | `String` | `McpResource.uri` |
| Prompt | `String` | `McpPrompt.name` |

## Thread Safety

- `MetadataMap` uses `RwLock<HashMap<K, V>>`
- Multiple concurrent reads allowed
- Exclusive write access for registration
- All handlers are `Send + Sync`

## Integration Points

### 1. Registration (register_tool)
```rust
pub async fn register_tool(
    &self,
    name: impl Into<String>,
    description: impl Into<String>,
    input_schema: Value,
    handler: McpToolHandler,
) {
    let definition = McpTool { name, description, input_schema };
    let metadata = ToolMetadata::new(definition, handler);
    self.tools.insert(name, metadata).await;
}
```

### 2. Listing (handle_list_tools)
```rust
async fn handle_list_tools(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
    let tools = self.list_tools().await; // Uses stored metadata
    let result = ListToolsResult { tools, next_cursor: None };
    serde_json::to_value(result)
}
```

### 3. Execution (handle_call_tool)
```rust
async fn handle_call_tool(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
    let metadata = self.tools.get(&params.name).await
        .ok_or_else(|| JsonRpcError::method_not_found(&params.name))?;
    
    match (metadata.handler)(params.arguments) {
        Ok(result) => serde_json::to_value(result),
        Err(e) => // error handling
    }
}
```

## Files

| File | Description |
|------|-------------|
| `docs/mcp_metadata_storage_design.md` | Full design document with rationale |
| `docs/mcp_metadata_storage_implementation.rs` | Reference implementation code |
| `docs/mcp_metadata_storage_summary.md` | This quick reference |

## Migration Steps

1. Add `MetadataMap`, `ToolMetadata`, `ResourceMetadata`, `PromptMetadata` types
2. Update `McpServer` fields to use new types
3. Update `register_tool`/`register_resource` to store complete metadata
4. Update handlers to retrieve from metadata storage
5. Remove hardcoded tool definitions from `handle_list_tools`

## Benefits

- ✅ Single source of truth for tool/resource info
- ✅ No duplication between registration and listing
- ✅ Support for introspection without execution
- ✅ Extensible custom metadata fields
- ✅ Thread-safe concurrent access
- ✅ Audit trail (registration timestamps)
