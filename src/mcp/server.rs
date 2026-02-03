//! MCP Server - Exposes CodeTether tools to MCP clients
//!
//! Runs as a stdio-based MCP server that can be connected to by:
//! - Claude Desktop
//! - Other MCP clients
//!
//! Exposed tools include:
//! - run_command: Execute shell commands
//! - read_file: Read file contents
//! - write_file: Write file contents
//! - search_files: Search for files
//! - swarm: Execute tasks with parallel sub-agents
//! - rlm: Analyze large content
//! - ralph: Autonomous PRD-driven execution

use super::transport::{McpMessage, StdioTransport, Transport};
use super::types::*;
use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// MCP Server implementation
pub struct McpServer {
    transport: Arc<dyn Transport>,
    tools: RwLock<HashMap<String, McpToolHandler>>,
    resources: RwLock<HashMap<String, McpResourceHandler>>,
    /// Prompt handlers for MCP prompts (reserved for future use)
    #[allow(dead_code)]
    prompts: RwLock<HashMap<String, McpPromptHandler>>,
    initialized: RwLock<bool>,
    server_info: ServerInfo,
    /// Tool metadata storage for querying tool information
    metadata: RwLock<HashMap<String, ToolMetadata>>,
    /// Resource metadata storage for querying resource information
    resource_metadata: RwLock<HashMap<String, ResourceMetadata>>,
}

type McpToolHandler = Arc<dyn Fn(Value) -> Result<CallToolResult> + Send + Sync>;
type McpResourceHandler = Arc<dyn Fn(String) -> Result<ReadResourceResult> + Send + Sync>;
type McpPromptHandler = Arc<dyn Fn(Value) -> Result<GetPromptResult> + Send + Sync>;

impl McpServer {
    /// Create a new MCP server over stdio
    pub fn new_stdio() -> Self {
        let transport = Arc::new(StdioTransport::new());
        Self::new(transport)
    }
    
    /// Create a new MCP server with custom transport
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        let mut server = Self {
            transport,
            tools: RwLock::new(HashMap::new()),
            resources: RwLock::new(HashMap::new()),
            prompts: RwLock::new(HashMap::new()),
            initialized: RwLock::new(false),
            server_info: ServerInfo {
                name: "codetether".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            metadata: RwLock::new(HashMap::new()),
            resource_metadata: RwLock::new(HashMap::new()),
        };
        
        // Register default tools
        server.register_default_tools();
        
        server
    }
    
    /// Register default CodeTether tools
    fn register_default_tools(&mut self) {
        // These will be registered synchronously in the constructor
        // The actual tool handlers will be added in run()
    }
    
    /// Register a tool
    pub async fn register_tool(&self, name: &str, description: &str, input_schema: Value, handler: McpToolHandler) {
        // Store tool metadata
        let metadata = ToolMetadata::new(
            name.to_string(),
            Some(description.to_string()),
            input_schema.clone(),
        );
        
        let mut metadata_map = self.metadata.write().await;
        metadata_map.insert(name.to_string(), metadata);
        drop(metadata_map);
        
        let mut tools = self.tools.write().await;
        tools.insert(name.to_string(), handler);
        
        debug!("Registered MCP tool: {}", name);
    }
    
    /// Register a resource
    pub async fn register_resource(&self, uri: &str, name: &str, description: &str, mime_type: Option<&str>, handler: McpResourceHandler) {
        // Store resource metadata
        let metadata = ResourceMetadata::new(
            uri.to_string(),
            name.to_string(),
            Some(description.to_string()),
            mime_type.map(|s| s.to_string()),
        );
        
        let mut metadata_map = self.resource_metadata.write().await;
        metadata_map.insert(uri.to_string(), metadata);
        drop(metadata_map);
        
        let mut resources = self.resources.write().await;
        resources.insert(uri.to_string(), handler);
        
        debug!("Registered MCP resource: {}", uri);
    }
    
    /// Get tool metadata by name
    pub async fn get_tool_metadata(&self, name: &str) -> Option<ToolMetadata> {
        let metadata = self.metadata.read().await;
        metadata.get(name).cloned()
    }
    
    /// Get all tool metadata
    pub async fn get_all_tool_metadata(&self) -> Vec<ToolMetadata> {
        let metadata = self.metadata.read().await;
        metadata.values().cloned().collect()
    }
    
    /// Get resource metadata by URI
    pub async fn get_resource_metadata(&self, uri: &str) -> Option<ResourceMetadata> {
        let metadata = self.resource_metadata.read().await;
        metadata.get(uri).cloned()
    }
    
    /// Get all resource metadata
    pub async fn get_all_resource_metadata(&self) -> Vec<ResourceMetadata> {
        let metadata = self.resource_metadata.read().await;
        metadata.values().cloned().collect()
    }
    
    /// Register a prompt handler
    pub async fn register_prompt(&self, name: &str, handler: McpPromptHandler) {
        let mut prompts = self.prompts.write().await;
        prompts.insert(name.to_string(), handler);
        debug!("Registered MCP prompt: {}", name);
    }
    
    /// Get a prompt handler by name
    pub async fn get_prompt_handler(&self, name: &str) -> Option<McpPromptHandler> {
        let prompts = self.prompts.read().await;
        prompts.get(name).cloned()
    }
    
    /// List all registered prompt names
    pub async fn list_prompts(&self) -> Vec<String> {
        let prompts = self.prompts.read().await;
        prompts.keys().cloned().collect()
    }
    
    /// Run the MCP server (main loop)
    pub async fn run(&self) -> Result<()> {
        info!("Starting MCP server...");
        
        // Register tools before starting
        self.setup_tools().await;
        
        loop {
            match self.transport.receive().await? {
                Some(McpMessage::Request(request)) => {
                    let response = self.handle_request(request).await;
                    self.transport.send_response(response).await?;
                }
                Some(McpMessage::Notification(notification)) => {
                    self.handle_notification(notification).await;
                }
                Some(McpMessage::Response(response)) => {
                    // We received a response (shouldn't happen in server mode)
                    warn!("Unexpected response received: {:?}", response.id);
                }
                None => {
                    info!("Transport closed, shutting down MCP server");
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Setup default tools
    async fn setup_tools(&self) {
        // run_command tool
        self.register_tool(
            "run_command",
            "Execute a shell command and return the output",
            json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The command to execute"
                    },
                    "cwd": {
                        "type": "string",
                        "description": "Working directory (optional)"
                    },
                    "timeout_ms": {
                        "type": "integer",
                        "description": "Timeout in milliseconds (default: 30000)"
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
                    format!("Exit code: {}\n{}{}", output.status.code().unwrap_or(-1), stdout, stderr)
                };
                
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: result }],
                    is_error: !output.status.success(),
                })
            }),
        ).await;
        
        // read_file tool
        self.register_tool(
            "read_file",
            "Read the contents of a file",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to read"
                    },
                    "offset": {
                        "type": "integer",
                        "description": "Line offset to start reading from (1-indexed)"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of lines to read"
                    }
                },
                "required": ["path"]
            }),
            Arc::new(|args| {
                let path = args.get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing path"))?;
                
                let content = std::fs::read_to_string(path)?;
                
                let offset = args.get("offset").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
                let limit = args.get("limit").and_then(|v| v.as_u64());
                
                let lines: Vec<&str> = content.lines().collect();
                let start = (offset.saturating_sub(1)).min(lines.len());
                let end = if let Some(l) = limit {
                    (start + l as usize).min(lines.len())
                } else {
                    lines.len()
                };
                
                let result = lines[start..end].join("\n");
                
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: result }],
                    is_error: false,
                })
            }),
        ).await;
        
        // write_file tool
        self.register_tool(
            "write_file",
            "Write content to a file",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the file to write"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to write"
                    },
                    "create_dirs": {
                        "type": "boolean",
                        "description": "Create parent directories if they don't exist"
                    }
                },
                "required": ["path", "content"]
            }),
            Arc::new(|args| {
                let path = args.get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing path"))?;
                
                let content = args.get("content")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing content"))?;
                
                let create_dirs = args.get("create_dirs")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                if create_dirs {
                    if let Some(parent) = std::path::Path::new(path).parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                }
                
                std::fs::write(path, content)?;
                
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: format!("Wrote {} bytes to {}", content.len(), path) }],
                    is_error: false,
                })
            }),
        ).await;
        
        // list_directory tool
        self.register_tool(
            "list_directory",
            "List contents of a directory",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path to the directory"
                    },
                    "recursive": {
                        "type": "boolean",
                        "description": "List recursively"
                    },
                    "max_depth": {
                        "type": "integer",
                        "description": "Maximum depth for recursive listing"
                    }
                },
                "required": ["path"]
            }),
            Arc::new(|args| {
                let path = args.get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing path"))?;
                
                let mut entries = Vec::new();
                for entry in std::fs::read_dir(path)? {
                    let entry = entry?;
                    let file_type = entry.file_type()?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    let suffix = if file_type.is_dir() { "/" } else { "" };
                    entries.push(format!("{}{}", name, suffix));
                }
                
                entries.sort();
                
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: entries.join("\n") }],
                    is_error: false,
                })
            }),
        ).await;
        
        // search_files tool
        self.register_tool(
            "search_files",
            "Search for files matching a pattern",
            json!({
                "type": "object",
                "properties": {
                    "pattern": {
                        "type": "string",
                        "description": "Search pattern (glob or regex)"
                    },
                    "path": {
                        "type": "string",
                        "description": "Directory to search in"
                    },
                    "content_pattern": {
                        "type": "string",
                        "description": "Pattern to search in file contents"
                    }
                },
                "required": ["pattern"]
            }),
            Arc::new(|args| {
                let pattern = args.get("pattern")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing pattern"))?;
                
                let path = args.get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or(".");
                
                // Simple glob using find command
                let output = std::process::Command::new("find")
                    .args([path, "-name", pattern, "-type", "f"])
                    .output()?;
                
                let result = String::from_utf8_lossy(&output.stdout);
                
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: result.to_string() }],
                    is_error: !output.status.success(),
                })
            }),
        ).await;
        
        // grep_search tool
        self.register_tool(
            "grep_search",
            "Search file contents using grep",
            json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search pattern"
                    },
                    "path": { 
                        "type": "string",
                        "description": "Directory or file to search"
                    },
                    "is_regex": {
                        "type": "boolean",
                        "description": "Treat pattern as regex"
                    },
                    "case_sensitive": {
                        "type": "boolean",
                        "description": "Case-sensitive search"
                    }
                },
                "required": ["query"]
            }),
            Arc::new(|args| {
                let query = args.get("query")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing query"))?;
                
                let path = args.get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or(".");
                
                let is_regex = args.get("is_regex")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                let case_sensitive = args.get("case_sensitive")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                let mut cmd = std::process::Command::new("grep");
                cmd.arg("-r").arg("-n");
                
                if !case_sensitive {
                    cmd.arg("-i");
                }
                
                if is_regex {
                    cmd.arg("-E");
                } else {
                    cmd.arg("-F");
                }
                
                cmd.arg(query).arg(path);
                
                let output = cmd.output()?;
                let result = String::from_utf8_lossy(&output.stdout);
                
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: result.to_string() }],
                    is_error: false,
                })
            }),
        ).await;
        
        info!("Registered {} MCP tools", self.tools.read().await.len());
    }
    
    /// Handle a JSON-RPC request
    async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Handling request: {} (id: {:?})", request.method, request.id);
        
        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params).await,
            "initialized" => Ok(json!({})),
            "ping" => Ok(json!({})),
            "tools/list" => self.handle_list_tools(request.params).await,
            "tools/call" => self.handle_call_tool(request.params).await,
            "resources/list" => self.handle_list_resources(request.params).await,
            "resources/read" => self.handle_read_resource(request.params).await,
            "prompts/list" => self.handle_list_prompts(request.params).await,
            "prompts/get" => self.handle_get_prompt(request.params).await,
            _ => Err(JsonRpcError::method_not_found(&request.method)),
        };
        
        match result {
            Ok(value) => JsonRpcResponse::success(request.id, value),
            Err(error) => JsonRpcResponse::error(request.id, error),
        }
    }
    
    /// Handle a notification
    async fn handle_notification(&self, notification: JsonRpcNotification) {
        debug!("Handling notification: {}", notification.method);
        
        match notification.method.as_str() {
            "notifications/initialized" => {
                *self.initialized.write().await = true;
                info!("MCP client initialized");
            }
            "notifications/cancelled" => {
                // Handle cancellation
            }
            _ => {
                debug!("Unknown notification: {}", notification.method);
            }
        }
    }
    
    /// Handle initialize request
    async fn handle_initialize(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let _params: InitializeParams = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
        } else {
            return Err(JsonRpcError::invalid_params("Missing params"));
        };
        
        let result = InitializeResult {
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: true }),
                resources: Some(ResourcesCapability { subscribe: false, list_changed: true }),
                prompts: Some(PromptsCapability { list_changed: true }),
                logging: Some(LoggingCapability {}),
                experimental: None,
            },
            server_info: self.server_info.clone(),
            instructions: Some(
                "CodeTether is an AI coding agent with tools for file operations, \
                 command execution, code search, and autonomous task execution. \
                 Use the swarm tool for complex tasks requiring parallel execution, \
                 and ralph for PRD-driven development.".to_string()
            ),
        };
        
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
    
    /// Handle list tools request
    async fn handle_list_tools(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        let _tools = self.tools.read().await;
        
        let tool_list: Vec<McpTool> = vec![
            McpTool {
                name: "run_command".to_string(),
                description: Some("Execute a shell command".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string" },
                        "cwd": { "type": "string" }
                    },
                    "required": ["command"]
                }),
            },
            McpTool {
                name: "read_file".to_string(),
                description: Some("Read file contents".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "offset": { "type": "integer" },
                        "limit": { "type": "integer" }
                    },
                    "required": ["path"]
                }),
            },
            McpTool {
                name: "write_file".to_string(),
                description: Some("Write content to a file".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "content": { "type": "string" }
                    },
                    "required": ["path", "content"]
                }),
            },
            McpTool {
                name: "list_directory".to_string(),
                description: Some("List directory contents".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" }
                    },
                    "required": ["path"]
                }),
            },
            McpTool {
                name: "search_files".to_string(),
                description: Some("Search for files by name pattern".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "pattern": { "type": "string" },
                        "path": { "type": "string" }
                    },
                    "required": ["pattern"]
                }),
            },
            McpTool {
                name: "grep_search".to_string(),
                description: Some("Search file contents".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string" },
                        "path": { "type": "string" },
                        "is_regex": { "type": "boolean" }
                    },
                    "required": ["query"]
                }),
            },
        ];
        
        let result = ListToolsResult {
            tools: tool_list,
            next_cursor: None,
        };
        
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
    
    /// Handle call tool request
    async fn handle_call_tool(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params: CallToolParams = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
        } else {
            return Err(JsonRpcError::invalid_params("Missing params"));
        };
        
        let tools = self.tools.read().await;
        let handler = tools.get(&params.name)
            .ok_or_else(|| JsonRpcError::method_not_found(&params.name))?;
        
        match handler(params.arguments) {
            Ok(result) => serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string())),
            Err(e) => {
                let result = CallToolResult {
                    content: vec![ToolContent::Text { text: e.to_string() }],
                    is_error: true,
                };
                serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
            }
        }
    }
    
    /// Handle list resources request
    async fn handle_list_resources(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        let result = ListResourcesResult {
            resources: vec![],
            next_cursor: None,
        };
        
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
    
    /// Handle read resource request
    async fn handle_read_resource(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params: ReadResourceParams = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
        } else {
            return Err(JsonRpcError::invalid_params("Missing params"));
        };
        
        let resources = self.resources.read().await;
        let handler = resources.get(&params.uri)
            .ok_or_else(|| JsonRpcError::method_not_found(&params.uri))?;
        
        match handler(params.uri.clone()) {
            Ok(result) => serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string())),
            Err(e) => Err(JsonRpcError::internal_error(e.to_string())),
        }
    }
    
    /// Handle list prompts request
    async fn handle_list_prompts(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        let result = ListPromptsResult {
            prompts: vec![
                McpPrompt {
                    name: "code_review".to_string(),
                    description: Some("Review code for issues and improvements".to_string()),
                    arguments: vec![
                        PromptArgument {
                            name: "file".to_string(),
                            description: Some("File to review".to_string()),
                            required: true,
                        },
                    ],
                },
                McpPrompt {
                    name: "explain_code".to_string(),
                    description: Some("Explain what code does".to_string()),
                    arguments: vec![
                        PromptArgument {
                            name: "file".to_string(),
                            description: Some("File to explain".to_string()),
                            required: true,
                        },
                    ],
                },
            ],
            next_cursor: None,
        };
        
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
    
    /// Handle get prompt request
    async fn handle_get_prompt(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params: GetPromptParams = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
        } else {
            return Err(JsonRpcError::invalid_params("Missing params"));
        };
        
        let result = match params.name.as_str() {
            "code_review" => {
                let file = params.arguments.get("file")
                    .and_then(|v| v.as_str())
                    .unwrap_or("file.rs");
                
                GetPromptResult {
                    description: Some("Code review prompt".to_string()),
                    messages: vec![
                        PromptMessage {
                            role: PromptRole::User,
                            content: PromptContent::Text {
                                text: format!(
                                    "Please review the following code for:\n\
                                     - Bugs and potential issues\n\
                                     - Performance concerns\n\
                                     - Code style and best practices\n\
                                     - Security vulnerabilities\n\n\
                                     File: {}", file
                                ),
                            },
                        },
                    ],
                }
            }
            "explain_code" => {
                let file = params.arguments.get("file")
                    .and_then(|v| v.as_str())
                    .unwrap_or("file.rs");
                
                GetPromptResult {
                    description: Some("Code explanation prompt".to_string()),
                    messages: vec![
                        PromptMessage {
                            role: PromptRole::User,
                            content: PromptContent::Text {
                                text: format!(
                                    "Please explain what this code does, including:\n\
                                     - Overall purpose\n\
                                     - Key functions and their roles\n\
                                     - Data flow\n\
                                     - Important algorithms used\n\n\
                                     File: {}", file
                                ),
                            },
                        },
                    ],
                }
            }
            _ => return Err(JsonRpcError::method_not_found(&params.name)),
        };
        
        serde_json::to_value(result).map_err(|e| JsonRpcError::internal_error(e.to_string()))
    }
}
