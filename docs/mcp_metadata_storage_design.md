# MCP Server Metadata Storage System Design

## Overview

This document describes the design for a metadata storage system for `src/mcp/server.rs` that stores 'tool' and 'resource' variables from `register_tool`/`register_resource` calls. The system provides a unified, thread-safe way to store and retrieve both metadata (descriptions, schemas) and handler references.

## Current State Analysis

### Existing Structure

The current `McpServer` stores handlers separately from metadata:

```rust
pub struct McpServer {
    transport: Arc<dyn Transport>,
    tools: RwLock<HashMap<String, McpToolHandler>>,      // Only stores handlers
    resources: RwLock<HashMap<String, McpResourceHandler>>, // Only stores handlers
    prompts: RwLock<HashMap<String, McpPromptHandler>>,
    initialized: RwLock<bool>,
    server_info: ServerInfo,
}

type McpToolHandler = Arc<dyn Fn(Value) -> Result<CallToolResult> + Send + Sync>;
type McpResourceHandler = Arc<dyn Fn(String) -> Result<ReadResourceResult> + Send + Sync>;
```

### Problems with Current Approach

1. **Metadata Loss**: `McpTool` and `McpResource` structs are created in `register_tool`/`register_resource` but only handlers are stored
2. **Duplication**: `handle_list_tools` hardcodes tool definitions instead of using registered metadata
3. **Inconsistency**: No single source of truth for tool/resource information
4. **No Introspection**: Cannot query metadata without invoking handlers

## Proposed Design

### Core Data Structures

#### 1. ToolMetadata

```rust
/// Complete metadata for a registered tool, including handler reference
#[derive(Clone)]
pub struct ToolMetadata {
    /// Tool definition for MCP protocol (name, description, schema)
    pub definition: McpTool,
    /// Handler function for executing the tool
    pub handler: McpToolHandler,
    /// Optional: Registration timestamp for debugging/auditing
    pub registered_at: std::time::Instant,
    /// Optional: Custom metadata for extensions
    pub custom: HashMap<String, Value>,
}

impl ToolMetadata {
    pub fn new(definition: McpTool, handler: McpToolHandler) -> Self {
        Self {
            definition,
            handler,
            registered_at: std::time::Instant::now(),
            custom: HashMap::new(),
        }
    }
    
    pub fn with_custom(mut self, key: impl Into<String>, value: Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }
}
```

#### 2. ResourceMetadata

```rust
/// Complete metadata for a registered resource, including handler reference
#[derive(Clone)]
pub struct ResourceMetadata {
    /// Resource definition for MCP protocol (uri, name, description, mime_type)
    pub definition: McpResource,
    /// Handler function for reading the resource
    pub handler: McpResourceHandler,
    /// Optional: Registration timestamp
    pub registered_at: std::time::Instant,
    /// Optional: Custom metadata for extensions
    pub custom: HashMap<String, Value>,
}

impl ResourceMetadata {
    pub fn new(definition: McpResource, handler: McpResourceHandler) -> Self {
        Self {
            definition,
            handler,
            registered_at: std::time::Instant::now(),
            custom: HashMap::new(),
        }
    }
    
    pub fn with_custom(mut self, key: impl Into<String>, value: Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }
}
```

#### 3. PromptMetadata

```rust
/// Complete metadata for a registered prompt, including handler reference
#[derive(Clone)]
pub struct PromptMetadata {
    /// Prompt definition for MCP protocol
    pub definition: McpPrompt,
    /// Handler function for generating the prompt
    pub handler: McpPromptHandler,
    /// Optional: Registration timestamp
    pub registered_at: std::time::Instant,
    /// Optional: Custom metadata for extensions
    pub custom: HashMap<String, Value>,
}

impl PromptMetadata {
    pub fn new(definition: McpPrompt, handler: McpPromptHandler) -> Self {
        Self {
            definition,
            handler,
            registered_at: std::time::Instant::now(),
            custom: HashMap::new(),
        }
    }
}
```

### MetadataMap Structure

```rust
/// Thread-safe metadata storage for MCP entities
pub struct MetadataMap<K, V> {
    inner: RwLock<HashMap<K, V>>,
}

impl<K, V> MetadataMap<K, V> 
where 
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
    
    /// Insert metadata into the map
    pub async fn insert(&self, key: K, value: V) {
        let mut map = self.inner.write().await;
        map.insert(key, value);
    }
    
    /// Get metadata by key
    pub async fn get(&self, key: &K) -> Option<V> {
        let map = self.inner.read().await;
        map.get(key).cloned()
    }
    
    /// Check if key exists
    pub async fn contains(&self, key: &K) -> bool {
        let map = self.inner.read().await;
        map.contains_key(key)
    }
    
    /// Remove metadata by key
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut map = self.inner.write().await;
        map.remove(key)
    }
    
    /// Get all values
    pub async fn values(&self) -> Vec<V> {
        let map = self.inner.read().await;
        map.values().cloned().collect()
    }
    
    /// Get count of entries
    pub async fn len(&self) -> usize {
        let map = self.inner.read().await;
        map.len()
    }
    
    /// Check if empty
    pub async fn is_empty(&self) -> bool {
        let map = self.inner.read().await;
        map.is_empty()
    }
}

// Default implementation
impl<K, V> Default for MetadataMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
```

### Updated McpServer Structure

```rust
/// MCP Server implementation with unified metadata storage
pub struct McpServer {
    transport: Arc<dyn Transport>,
    /// Tool metadata storage (keyed by tool name)
    tools: MetadataMap<String, ToolMetadata>,
    /// Resource metadata storage (keyed by resource URI)
    resources: MetadataMap<String, ResourceMetadata>,
    /// Prompt metadata storage (keyed by prompt name)
    prompts: MetadataMap<String, PromptMetadata>,
    /// Server initialization state
    initialized: RwLock<bool>,
    /// Server information for protocol handshake
    server_info: ServerInfo,
}
```

## Type Signatures

### Handler Type Aliases

```rust
/// Tool handler type: takes JSON arguments, returns tool result
pub type McpToolHandler = Arc<
    dyn Fn(Value) -> Result<CallToolResult, anyhow::Error> + Send + Sync
>;

/// Resource handler type: takes URI string, returns resource contents
pub type McpResourceHandler = Arc<
    dyn Fn(String) -> Result<ReadResourceResult, anyhow::Error> + Send + Sync
>;

/// Prompt handler type: takes JSON arguments, returns prompt result
pub type McpPromptHandler = Arc<
    dyn Fn(Value) -> Result<GetPromptResult, anyhow::Error> + Send + Sync
>;
```

### Registration Methods

```rust
impl McpServer {
    /// Register a tool with its complete metadata
    /// 
    /// # Arguments
    /// * `name` - Unique tool identifier
    /// * `description` - Human-readable description
    /// * `input_schema` - JSON Schema for input validation
    /// * `handler` - Async handler function
    pub async fn register_tool(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: Value,
        handler: McpToolHandler,
    );
    
    /// Register a resource with its complete metadata
    /// 
    /// # Arguments
    /// * `uri` - Unique resource URI
    /// * `name` - Human-readable name
    /// * `description` - Human-readable description
    /// * `mime_type` - Optional MIME type
    /// * `handler` - Async handler function
    pub async fn register_resource(
        &self,
        uri: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        mime_type: Option<impl Into<String>>,
        handler: McpResourceHandler,
    );
    
    /// Register a prompt with its complete metadata
    /// 
    /// # Arguments
    /// * `name` - Unique prompt identifier
    /// * `description` - Human-readable description
    /// * `arguments` - List of prompt arguments
    /// * `handler` - Async handler function
    pub async fn register_prompt(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        arguments: Vec<PromptArgument>,
        handler: McpPromptHandler,
    );
}
```

### Query Methods

```rust
impl McpServer {
    /// Get tool metadata by name
    pub async fn get_tool(&self, name: &str) -> Option<ToolMetadata>;
    
    /// Get resource metadata by URI
    pub async fn get_resource(&self, uri: &str) -> Option<ResourceMetadata>;
    
    /// Get prompt metadata by name
    pub async fn get_prompt(&self, name: &str) -> Option<PromptMetadata>;
    
    /// List all registered tools
    pub async fn list_tools(&self) -> Vec<McpTool>;
    
    /// List all registered resources
    pub async fn list_resources(&self) -> Vec<McpResource>;
    
    /// List all registered prompts
    pub async fn list_prompts(&self) -> Vec<McpPrompt>;
    
    /// Check if a tool is registered
    pub async fn has_tool(&self, name: &str) -> bool;
    
    /// Check if a resource is registered
    pub async fn has_resource(&self, uri: &str) -> bool;
    
    /// Check if a prompt is registered
    pub async fn has_prompt(&self, name: &str) -> bool;
    
    /// Get count of registered tools
    pub async fn tool_count(&self) -> usize;
    
    /// Get count of registered resources
    pub async fn resource_count(&self) -> usize;
    
    /// Get count of registered prompts
    pub async fn prompt_count(&self) -> usize;
}
```

## Thread Safety

### Requirements

1. **Concurrent Reads**: Multiple handlers can read metadata simultaneously
2. **Exclusive Writes**: Registration requires exclusive access
3. **Handler Safety**: Handlers are `Send + Sync` for use across threads
4. **No Deadlocks**: Lock ordering: tools -> resources -> prompts

### Implementation

```rust
// MetadataMap uses RwLock for concurrent access
pub struct MetadataMap<K, V> {
    inner: RwLock<HashMap<K, V>>,
}

// All metadata types are Clone + Send + Sync
unsafe impl Send for ToolMetadata {}
unsafe impl Sync for ToolMetadata {}
unsafe impl Send for ResourceMetadata {}
unsafe impl Sync for ResourceMetadata {}
unsafe impl Send for PromptMetadata {}
unsafe impl Sync for PromptMetadata {}
```

## Integration Points

### 1. Server Construction

```rust
impl McpServer {
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        let server = Self {
            transport,
            tools: MetadataMap::new(),
            resources: MetadataMap::new(),
            prompts: MetadataMap::new(),
            initialized: RwLock::new(false),
            server_info: ServerInfo {
                name: "codetether".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };
        
        // Register default tools
        server.setup_default_tools();
        
        server
    }
}
```

### 2. Tool Registration

```rust
pub async fn register_tool(
    &self,
    name: impl Into<String>,
    description: impl Into<String>,
    input_schema: Value,
    handler: McpToolHandler,
) {
    let name = name.into();
    let description = description.into();
    
    let definition = McpTool {
        name: name.clone(),
        description: Some(description),
        input_schema,
    };
    
    let metadata = ToolMetadata::new(definition, handler);
    self.tools.insert(name.clone(), metadata).await;
    
    debug!("Registered MCP tool: {}", name);
}
```

### 3. List Tools Handler

```rust
async fn handle_list_tools(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
    let tools = self.list_tools().await;
    
    let result = ListToolsResult {
        tools,
        next_cursor: None,
    };
    
    serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
}
```

### 4. Call Tool Handler

```rust
async fn handle_call_tool(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
    let params: CallToolParams = if let Some(p) = params {
        serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
    } else {
        return Err(JsonRpcError::invalid_params("Missing params"));
    };
    
    let metadata = self.tools.get(&params.name).await
        .ok_or_else(|| JsonRpcError::method_not_found(&params.name))?;
    
    match (metadata.handler)(params.arguments) {
        Ok(result) => serde_json::to_value(result)
            .map_err(|e| JsonRpcError::internal_error(e.to_string())),
        Err(e) => {
            let result = CallToolResult {
                content: vec![ToolContent::Text { text: e.to_string() }],
                is_error: true,
            };
            serde_json::to_value(result)
                .map_err(|e| JsonRpcError::internal_error(e.to_string()))
        }
    }
}
```

## Migration Strategy

### Phase 1: Add Metadata Types

1. Add `ToolMetadata`, `ResourceMetadata`, `PromptMetadata` to `src/mcp/server.rs`
2. Add `MetadataMap` wrapper type
3. Keep existing `HashMap` fields for backward compatibility

### Phase 2: Update Registration

1. Modify `register_tool` to create and store `ToolMetadata`
2. Modify `register_resource` to create and store `ResourceMetadata`
3. Add `register_prompt` for completeness

### Phase 3: Update Handlers

1. Update `handle_list_tools` to use stored metadata
2. Update `handle_list_resources` to use stored metadata
3. Update `handle_list_prompts` to use stored metadata
4. Remove hardcoded tool/resource definitions

### Phase 4: Cleanup

1. Remove old `HashMap` fields
2. Update all references to use new `MetadataMap` fields
3. Add comprehensive tests

## Usage Examples

### Registering a Tool

```rust
server.register_tool(
    "read_file",
    "Read the contents of a file",
    json!({
        "type": "object",
        "properties": {
            "path": { "type": "string" }
        },
        "required": ["path"]
    }),
    Arc::new(|args| {
        // Handler implementation
        Ok(CallToolResult { ... })
    }),
).await;
```

### Querying Metadata

```rust
// Get tool metadata
if let Some(metadata) = server.get_tool("read_file").await {
    println!("Tool: {}", metadata.definition.name);
    println!("Description: {:?}", metadata.definition.description);
    println!("Registered: {:?}", metadata.registered_at);
}

// List all tools
let tools = server.list_tools().await;
for tool in tools {
    println!("- {}: {:?}", tool.name, tool.description);
}
```

### Dynamic Tool Discovery

```rust
// Server can now support dynamic tool registration
pub async fn register_dynamic_tool(
    &self,
    name: &str,
    config: ToolConfig,
) -> Result<(), Error> {
    // Validate config
    let schema = generate_schema(&config)?;
    let handler = create_handler(&config)?;
    
    self.register_tool(name, config.description, schema, handler).await;
    
    // Notify clients of change if supported
    if self.supports_tool_list_changed().await {
        self.notify_tool_list_changed().await?;
    }
    
    Ok(())
}
```

## Benefits

1. **Single Source of Truth**: Metadata and handlers stored together
2. **No Duplication**: `handle_list_tools` uses stored metadata
3. **Introspection**: Can query tool/resource info without execution
4. **Extensibility**: Custom metadata field allows extensions
5. **Type Safety**: Strongly typed metadata structures
6. **Thread Safety**: `RwLock` allows concurrent reads
7. **Testability**: Easy to mock and test metadata operations

## Future Extensions

1. **Hot Reloading**: Support for updating handlers at runtime
2. **Tool Versioning**: Track tool versions for compatibility
3. **Usage Metrics**: Track call counts and timing
4. **Access Control**: Add permission metadata to restrict tool access
5. **Caching**: Cache resource contents with TTL
6. **Subscriptions**: Support for resource change notifications
