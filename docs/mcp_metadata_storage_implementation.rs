// Reference Implementation: MCP Server Metadata Storage System
// This file shows the complete implementation of the design
// Copy relevant sections into src/mcp/server.rs

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Instant;
use serde_json::Value;
use tokio::sync::RwLock;
use anyhow::Result;

// Import existing types from src/mcp/types.rs
use crate::mcp::types::*;

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// Tool handler type: takes JSON arguments, returns tool result
pub type McpToolHandler = Arc<
    dyn Fn(Value) -> Result<CallToolResult> + Send + Sync
>;

/// Resource handler type: takes URI string, returns resource contents
pub type McpResourceHandler = Arc<
    dyn Fn(String) -> Result<ReadResourceResult> + Send + Sync
>;

/// Prompt handler type: takes JSON arguments, returns prompt result
pub type McpPromptHandler = Arc<
    dyn Fn(Value) -> Result<GetPromptResult> + Send + Sync
>;

// ============================================================================
// METADATA STRUCTURES
// ============================================================================

/// Complete metadata for a registered tool, including handler reference
#[derive(Clone)]
pub struct ToolMetadata {
    /// Tool definition for MCP protocol (name, description, schema)
    pub definition: McpTool,
    /// Handler function for executing the tool
    pub handler: McpToolHandler,
    /// Registration timestamp for debugging/auditing
    pub registered_at: Instant,
    /// Custom metadata for extensions
    pub custom: HashMap<String, Value>,
}

impl ToolMetadata {
    /// Create new tool metadata
    pub fn new(definition: McpTool, handler: McpToolHandler) -> Self {
        Self {
            definition,
            handler,
            registered_at: Instant::now(),
            custom: HashMap::new(),
        }
    }
    
    /// Add custom metadata
    pub fn with_custom(mut self, key: impl Into<String>, value: Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }
    
    /// Get tool name
    pub fn name(&self) -> &str {
        &self.definition.name
    }
    
    /// Get tool description
    pub fn description(&self) -> Option<&str> {
        self.definition.description.as_deref()
    }
}

/// Complete metadata for a registered resource, including handler reference
#[derive(Clone)]
pub struct ResourceMetadata {
    /// Resource definition for MCP protocol (uri, name, description, mime_type)
    pub definition: McpResource,
    /// Handler function for reading the resource
    pub handler: McpResourceHandler,
    /// Registration timestamp
    pub registered_at: Instant,
    /// Custom metadata for extensions
    pub custom: HashMap<String, Value>,
}

impl ResourceMetadata {
    /// Create new resource metadata
    pub fn new(definition: McpResource, handler: McpResourceHandler) -> Self {
        Self {
            definition,
            handler,
            registered_at: Instant::now(),
            custom: HashMap::new(),
        }
    }
    
    /// Add custom metadata
    pub fn with_custom(mut self, key: impl Into<String>, value: Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }
    
    /// Get resource URI
    pub fn uri(&self) -> &str {
        &self.definition.uri
    }
    
    /// Get resource name
    pub fn name(&self) -> &str {
        &self.definition.name
    }
}

/// Complete metadata for a registered prompt, including handler reference
#[derive(Clone)]
pub struct PromptMetadata {
    /// Prompt definition for MCP protocol
    pub definition: McpPrompt,
    /// Handler function for generating the prompt
    pub handler: McpPromptHandler,
    /// Registration timestamp
    pub registered_at: Instant,
    /// Custom metadata for extensions
    pub custom: HashMap<String, Value>,
}

impl PromptMetadata {
    /// Create new prompt metadata
    pub fn new(definition: McpPrompt, handler: McpPromptHandler) -> Self {
        Self {
            definition,
            handler,
            registered_at: Instant::now(),
            custom: HashMap::new(),
        }
    }
    
    /// Add custom metadata
    pub fn with_custom(mut self, key: impl Into<String>, value: Value) -> Self {
        self.custom.insert(key.into(), value);
        self
    }
    
    /// Get prompt name
    pub fn name(&self) -> &str {
        &self.definition.name
    }
}

// ============================================================================
// METADATA MAP
// ============================================================================

/// Thread-safe metadata storage for MCP entities
pub struct MetadataMap<K, V> {
    inner: RwLock<HashMap<K, V>>,
}

impl<K, V> MetadataMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new empty metadata map
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
    
    /// Get all keys
    pub async fn keys(&self) -> Vec<K> {
        let map = self.inner.read().await;
        map.keys().cloned().collect()
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
    
    /// Clear all entries
    pub async fn clear(&self) {
        let mut map = self.inner.write().await;
        map.clear();
    }
    
    /// Update an entry if it exists
    pub async fn update<F>(&self, key: &K, f: F) -> bool
    where
        F: FnOnce(&mut V),
    {
        let mut map = self.inner.write().await;
        if let Some(value) = map.get_mut(key) {
            f(value);
            true
        } else {
            false
        }
    }
}

impl<K, V> Default for MetadataMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UPDATED MCP SERVER
// ============================================================================

use super::transport::{McpMessage, StdioTransport, Transport};
use tracing::{debug, info, warn};

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

impl McpServer {
    /// Create a new MCP server over stdio
    pub fn new_stdio() -> Self {
        let transport = Arc::new(StdioTransport::new());
        Self::new(transport)
    }
    
    /// Create a new MCP server with custom transport
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
        
        // Note: setup_default_tools is called in run() since we need async
        
        server
    }
    
    // ========================================================================
    // REGISTRATION METHODS
    // ========================================================================
    
    /// Register a tool with its complete metadata
    /// 
    /// # Arguments
    /// * `name` - Unique tool identifier
    /// * `description` - Human-readable description
    /// * `input_schema` - JSON Schema for input validation
    /// * `handler` - Handler function
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
    
    /// Register a resource with its complete metadata
    /// 
    /// # Arguments
    /// * `uri` - Unique resource URI
    /// * `name` - Human-readable name
    /// * `description` - Human-readable description
    /// * `mime_type` - Optional MIME type
    /// * `handler` - Handler function
    pub async fn register_resource(
        &self,
        uri: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        mime_type: Option<impl Into<String>>,
        handler: McpResourceHandler,
    ) {
        let uri = uri.into();
        let name = name.into();
        let description = description.into();
        
        let definition = McpResource {
            uri: uri.clone(),
            name,
            description: Some(description),
            mime_type: mime_type.map(|s| s.into()),
        };
        
        let metadata = ResourceMetadata::new(definition, handler);
        self.resources.insert(uri.clone(), metadata).await;
        
        debug!("Registered MCP resource: {}", uri);
    }
    
    /// Register a prompt with its complete metadata
    /// 
    /// # Arguments
    /// * `name` - Unique prompt identifier
    /// * `description` - Human-readable description
    /// * `arguments` - List of prompt arguments
    /// * `handler` - Handler function
    pub async fn register_prompt(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        arguments: Vec<PromptArgument>,
        handler: McpPromptHandler,
    ) {
        let name = name.into();
        let description = description.into();
        
        let definition = McpPrompt {
            name: name.clone(),
            description: Some(description),
            arguments,
        };
        
        let metadata = PromptMetadata::new(definition, handler);
        self.prompts.insert(name.clone(), metadata).await;
        
        debug!("Registered MCP prompt: {}", name);
    }
    
    // ========================================================================
    // QUERY METHODS
    // ========================================================================
    
    /// Get tool metadata by name
    pub async fn get_tool(&self, name: &str) -> Option<ToolMetadata> {
        self.tools.get(&name.to_string()).await
    }
    
    /// Get resource metadata by URI
    pub async fn get_resource(&self, uri: &str) -> Option<ResourceMetadata> {
        self.resources.get(&uri.to_string()).await
    }
    
    /// Get prompt metadata by name
    pub async fn get_prompt(&self, name: &str) -> Option<PromptMetadata> {
        self.prompts.get(&name.to_string()).await
    }
    
    /// List all registered tools (returns definitions only)
    pub async fn list_tools(&self) -> Vec<McpTool> {
        self.tools
            .values()
            .await
            .into_iter()
            .map(|m| m.definition)
            .collect()
    }
    
    /// List all registered resources (returns definitions only)
    pub async fn list_resources(&self) -> Vec<McpResource> {
        self.resources
            .values()
            .await
            .into_iter()
            .map(|m| m.definition)
            .collect()
    }
    
    /// List all registered prompts (returns definitions only)
    pub async fn list_prompts(&self) -> Vec<McpPrompt> {
        self.prompts
            .values()
            .await
            .into_iter()
            .map(|m| m.definition)
            .collect()
    }
    
    /// Check if a tool is registered
    pub async fn has_tool(&self, name: &str) -> bool {
        self.tools.contains(&name.to_string()).await
    }
    
    /// Check if a resource is registered
    pub async fn has_resource(&self, uri: &str) -> bool {
        self.resources.contains(&uri.to_string()).await
    }
    
    /// Check if a prompt is registered
    pub async fn has_prompt(&self, name: &str) -> bool {
        self.prompts.contains(&name.to_string()).await
    }
    
    /// Get count of registered tools
    pub async fn tool_count(&self) -> usize {
        self.tools.len().await
    }
    
    /// Get count of registered resources
    pub async fn resource_count(&self) -> usize {
        self.resources.len().await
    }
    
    /// Get count of registered prompts
    pub async fn prompt_count(&self) -> usize {
        self.prompts.len().await
    }
    
    // ========================================================================
    // REQUEST HANDLERS (Updated to use metadata storage)
    // ========================================================================
    
    /// Handle list tools request - uses stored metadata
    async fn handle_list_tools(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        let tools = self.list_tools().await;
        
        let result = ListToolsResult {
            tools,
            next_cursor: None,
        };
        
        serde_json::to_value(result)
            .map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
    
    /// Handle call tool request - retrieves handler from metadata
    async fn handle_call_tool(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params: CallToolParams = if let Some(p) = params {
            serde_json::from_value(p)
                .map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
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
    
    /// Handle list resources request - uses stored metadata
    async fn handle_list_resources(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        let resources = self.list_resources().await;
        
        let result = ListResourcesResult {
            resources,
            next_cursor: None,
        };
        
        serde_json::to_value(result)
            .map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
    
    /// Handle read resource request - retrieves handler from metadata
    async fn handle_read_resource(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params: ReadResourceParams = if let Some(p) = params {
            serde_json::from_value(p)
                .map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
        } else {
            return Err(JsonRpcError::invalid_params("Missing params"));
        };
        
        let metadata = self.resources.get(&params.uri).await
            .ok_or_else(|| JsonRpcError::method_not_found(&params.uri))?;
        
        match (metadata.handler)(params.uri) {
            Ok(result) => serde_json::to_value(result)
                .map_err(|e| JsonRpcError::internal_error(e.to_string())),
            Err(e) => Err(JsonRpcError::internal_error(e.to_string())),
        }
    }
    
    /// Handle list prompts request - uses stored metadata
    async fn handle_list_prompts(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        let prompts = self.list_prompts().await;
        
        let result = ListPromptsResult {
            prompts,
            next_cursor: None,
        };
        
        serde_json::to_value(result)
            .map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
    
    /// Handle get prompt request - retrieves handler from metadata
    async fn handle_get_prompt(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params: GetPromptParams = if let Some(p) = params {
            serde_json::from_value(p)
                .map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
        } else {
            return Err(JsonRpcError::invalid_params("Missing params"));
        };
        
        let metadata = self.prompts.get(&params.name).await
            .ok_or_else(|| JsonRpcError::method_not_found(&params.name))?;
        
        match (metadata.handler)(params.arguments) {
            Ok(result) => serde_json::to_value(result)
                .map_err(|e| JsonRpcError::internal_error(e.to_string())),
            Err(e) => Err(JsonRpcError::internal_error(e.to_string())),
        }
    }
}

// ============================================================================
// EXAMPLE: SETUP DEFAULT TOOLS
// ============================================================================

impl McpServer {
    /// Setup default tools - updated to use new registration
    async fn setup_tools(&self) {
        // run_command tool
        self.register_tool(
            "run_command",
            "Execute a shell command and return the output",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The command to execute"
                    },
                    "cwd": {
                        "type": "string",
                        "description": "Working directory (optional)"
                    }
                },
                "required": ["command"]
            }),
            Arc::new(|args| {
                let command = args.get("command")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing command"))?;
                
                let cwd = args.get("cwd").and_then(|v| v.as_str());
                
                let mut cmd = std::process::Command::new("/bin/sh");
                cmd.arg("-c").arg(command);
                
                if let Some(dir) = cwd {
                    cmd.current_dir(dir);
                }
                
                let output = cmd.output()?;
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                let result = if output.status.success() {
                    format!("{}{}", stdout, stderr)
                } else {
                    format!("Exit code: {}\n{}{}", 
                        output.status.code().unwrap_or(-1), 
                        stdout, 
                        stderr
                    )
                };
                
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: result }],
                    is_error: !output.status.success(),
                })
            }),
        ).await;
        
        info!("Registered {} MCP tools", self.tool_count().await);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_metadata_map_insert_and_get() {
        let map: MetadataMap<String, ToolMetadata> = MetadataMap::new();
        
        let definition = McpTool {
            name: "test_tool".to_string(),
            description: Some("A test tool".to_string()),
            input_schema: Value::Null,
        };
        
        let handler: McpToolHandler = Arc::new(|_| {
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: "ok".to_string() }],
                is_error: false,
            })
        });
        
        let metadata = ToolMetadata::new(definition, handler);
        map.insert("test_tool".to_string(), metadata.clone()).await;
        
        let retrieved = map.get(&"test_tool".to_string()).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test_tool");
    }
    
    #[tokio::test]
    async fn test_metadata_map_contains() {
        let map: MetadataMap<String, ToolMetadata> = MetadataMap::new();
        
        assert!(!map.contains(&"missing".to_string()).await);
        
        let definition = McpTool {
            name: "exists".to_string(),
            description: None,
            input_schema: Value::Null,
        };
        
        let handler: McpToolHandler = Arc::new(|_| {
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: "ok".to_string() }],
                is_error: false,
            })
        });
        
        let metadata = ToolMetadata::new(definition, handler);
        map.insert("exists".to_string(), metadata).await;
        
        assert!(map.contains(&"exists".to_string()).await);
    }
    
    #[tokio::test]
    async fn test_tool_metadata_custom_fields() {
        let definition = McpTool {
            name: "test".to_string(),
            description: Some("Test".to_string()),
            input_schema: Value::Null,
        };
        
        let handler: McpToolHandler = Arc::new(|_| {
            Ok(CallToolResult {
                content: vec![ToolContent::Text { text: "ok".to_string() }],
                is_error: false,
            })
        });
        
        let metadata = ToolMetadata::new(definition, handler)
            .with_custom("version", Value::String("1.0".to_string()))
            .with_custom("author", Value::String("test".to_string()));
        
        assert_eq!(
            metadata.custom.get("version"),
            Some(&Value::String("1.0".to_string()))
        );
    }
}
