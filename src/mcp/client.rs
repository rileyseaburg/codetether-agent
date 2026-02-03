//! MCP Client - Connect to external MCP servers
//!
//! Allows CodeTether to use tools from other MCP servers:
//! - Filesystem servers
//! - Database servers
//! - API integration servers
//! - Custom tool servers

use super::transport::{McpMessage, ProcessTransport, Transport};
use super::types::*;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{oneshot, RwLock};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

/// MCP Client for connecting to external servers
pub struct McpClient {
    transport: Arc<dyn Transport>,
    pending_requests: RwLock<HashMap<RequestId, oneshot::Sender<JsonRpcResponse>>>,
    request_id: AtomicI64,
    server_info: RwLock<Option<ServerInfo>>,
    server_capabilities: RwLock<Option<ServerCapabilities>>,
    available_tools: RwLock<Vec<McpTool>>,
    /// Registry for managing multiple MCP server connections and capability tracking
    registry: Arc<McpRegistry>,
    /// Server name identifier for registry tracking
    server_name: RwLock<Option<String>>,
}

impl McpClient {
    /// Connect to an MCP server via subprocess
    pub async fn connect_subprocess(command: &str, args: &[&str]) -> Result<Arc<Self>> {
        let transport = Arc::new(ProcessTransport::spawn(command, args).await?);
        let client = Arc::new(Self::new(transport));
        
        // Start message receiver
        let client_clone = Arc::clone(&client);
        tokio::spawn(async move {
            client_clone.receive_loop().await;
        });
        
        // Initialize the connection
        client.initialize().await?;
        
        Ok(client)
    }
    
    /// Create a new MCP client with custom transport
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self {
            transport,
            pending_requests: RwLock::new(HashMap::new()),
            request_id: AtomicI64::new(1),
            server_info: RwLock::new(None),
            server_capabilities: RwLock::new(None),
            available_tools: RwLock::new(Vec::new()),
            registry: Arc::new(McpRegistry::new()),
            server_name: RwLock::new(None),
        }
    }

    /// Create a new MCP client with a shared registry for multi-server management
    pub fn with_registry(transport: Arc<dyn Transport>, registry: Arc<McpRegistry>, name: Option<String>) -> Self {
        Self {
            transport,
            pending_requests: RwLock::new(HashMap::new()),
            request_id: AtomicI64::new(1),
            server_info: RwLock::new(None),
            server_capabilities: RwLock::new(None),
            available_tools: RwLock::new(Vec::new()),
            registry,
            server_name: RwLock::new(name),
        }
    }
    
    /// Initialize the connection with the server
    pub async fn initialize(&self) -> Result<InitializeResult> {
        let params = InitializeParams {
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities: ClientCapabilities {
                roots: Some(RootsCapability { list_changed: true }),
                sampling: Some(SamplingCapability {}),
                experimental: None,
            },
            client_info: ClientInfo {
                name: "codetether".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };
        
        let response = self.request("initialize", Some(serde_json::to_value(&params)?)).await?;
        let result: InitializeResult = serde_json::from_value(response)?;
        
        // Store server info
        *self.server_info.write().await = Some(result.server_info.clone());
        *self.server_capabilities.write().await = Some(result.capabilities.clone());
        
        // Register this client with the registry if a server name is set
        if let Some(name) = self.server_name.read().await.clone() {
            // Create a self-reference for registration
            // Note: This is called after construction, so we need to handle registration externally
            // or use a post-initialization hook
            debug!("Client initialized with server name: {}", name);
        }
        
        // Send initialized notification
        self.notify("notifications/initialized", None).await?;
        
        info!(
            "Connected to MCP server: {} v{}",
            result.server_info.name, result.server_info.version
        );
        
        // Fetch available tools
        if result.capabilities.tools.is_some() {
            self.refresh_tools().await?;
        }
        
        Ok(result)
    }
    
    /// Get the registry associated with this client
    pub fn registry(&self) -> Arc<McpRegistry> {
        Arc::clone(&self.registry)
    }
    
    /// Get the server name if set
    pub async fn server_name(&self) -> Option<String> {
        self.server_name.read().await.clone()
    }
    
    /// Set the server name for registry tracking
    pub async fn set_server_name(&self, name: String) {
        *self.server_name.write().await = Some(name);
    }
    
    /// Check if the connected server has a specific capability
    pub async fn has_capability(&self, capability: &str) -> bool {
        let caps = self.server_capabilities.read().await;
        match capability {
            "tools" => caps.as_ref().map(|c| c.tools.is_some()).unwrap_or(false),
            "resources" => caps.as_ref().map(|c| c.resources.is_some()).unwrap_or(false),
            "prompts" => caps.as_ref().map(|c| c.prompts.is_some()).unwrap_or(false),
            "logging" => caps.as_ref().map(|c| c.logging.is_some()).unwrap_or(false),
            _ => false,
        }
    }
    
    /// Get server capabilities
    pub async fn capabilities(&self) -> Option<ServerCapabilities> {
        self.server_capabilities.read().await.clone()
    }
    
    /// Discover tools from the registry across all connected servers
    pub async fn discover_tools_from_registry(&self) -> Vec<(String, McpTool)> {
        self.registry.all_tools().await
    }
    
    /// Find a tool across all servers in the registry
    pub async fn find_tool_in_registry(&self, tool_name: &str) -> Option<(String, McpTool)> {
        self.registry.find_tool(tool_name).await
    }
    
    /// Refresh the list of available tools
    pub async fn refresh_tools(&self) -> Result<Vec<McpTool>> {
        let response = self.request("tools/list", None).await?;
        let result: ListToolsResult = serde_json::from_value(response)?;
        
        *self.available_tools.write().await = result.tools.clone();
        
        info!("Loaded {} tools from MCP server", result.tools.len());
        
        Ok(result.tools)
    }
    
    /// Get available tools
    pub async fn tools(&self) -> Vec<McpTool> {
        self.available_tools.read().await.clone()
    }
    
    /// Call a tool
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        let params = CallToolParams {
            name: name.to_string(),
            arguments,
        };
        
        let response = self.request("tools/call", Some(serde_json::to_value(&params)?)).await?;
        let result: CallToolResult = serde_json::from_value(response)?;
        
        Ok(result)
    }
    
    /// List available resources
    pub async fn list_resources(&self) -> Result<Vec<McpResource>> {
        let response = self.request("resources/list", None).await?;
        let result: ListResourcesResult = serde_json::from_value(response)?;
        Ok(result.resources)
    }
    
    /// Read a resource
    pub async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult> {
        let params = ReadResourceParams { uri: uri.to_string() };
        let response = self.request("resources/read", Some(serde_json::to_value(&params)?)).await?;
        let result: ReadResourceResult = serde_json::from_value(response)?;
        Ok(result)
    }
    
    /// List available prompts
    pub async fn list_prompts(&self) -> Result<Vec<McpPrompt>> {
        let response = self.request("prompts/list", None).await?;
        let result: ListPromptsResult = serde_json::from_value(response)?;
        Ok(result.prompts)
    }
    
    /// Get a prompt
    pub async fn get_prompt(&self, name: &str, arguments: Value) -> Result<GetPromptResult> {
        let params = GetPromptParams {
            name: name.to_string(),
            arguments,
        };
        let response = self.request("prompts/get", Some(serde_json::to_value(&params)?)).await?;
        let result: GetPromptResult = serde_json::from_value(response)?;
        Ok(result)
    }
    
    /// Send a JSON-RPC request and wait for response
    async fn request(&self, method: &str, params: Option<Value>) -> Result<Value> {
        let id = RequestId::Number(self.request_id.fetch_add(1, Ordering::SeqCst));
        let request = JsonRpcRequest::new(id.clone(), method, params);
        
        // Create response channel
        let (tx, rx) = oneshot::channel();
        self.pending_requests.write().await.insert(id.clone(), tx);
        
        // Send request
        self.transport.send_request(request).await?;
        
        // Wait for response with timeout
        let response = timeout(Duration::from_secs(30), rx)
            .await
            .map_err(|_| anyhow::anyhow!("Request timed out"))??;
        
        if let Some(error) = response.error {
            return Err(anyhow::anyhow!("MCP error {}: {}", error.code, error.message));
        }
        
        response.result.ok_or_else(|| anyhow::anyhow!("Empty response"))
    }
    
    /// Send a notification (no response expected)
    async fn notify(&self, method: &str, params: Option<Value>) -> Result<()> {
        let notification = JsonRpcNotification::new(method, params);
        self.transport.send_notification(notification).await
    }
    
    /// Message receive loop
    async fn receive_loop(&self) {
        loop {
            match self.transport.receive().await {
                Ok(Some(message)) => {
                    self.handle_message(message).await;
                }
                Ok(None) => {
                    info!("MCP connection closed");
                    break;
                }
                Err(e) => {
                    error!("Error receiving MCP message: {}", e);
                    break;
                }
            }
        }
    }
    
    /// Handle an incoming message
    async fn handle_message(&self, message: McpMessage) {
        match message {
            McpMessage::Response(response) => {
                // Find and notify the waiting request
                if let Some(tx) = self.pending_requests.write().await.remove(&response.id) {
                    let _ = tx.send(response);
                } else {
                    warn!("Received response for unknown request: {:?}", response.id);
                }
            }
            McpMessage::Request(request) => {
                // Server is making a request to us (e.g., sampling)
                debug!("Received request from server: {}", request.method);
                
                let response = match request.method.as_str() {
                    "sampling/createMessage" => {
                        // Handle sampling request
                        // TODO: Implement sampling using our provider
                        JsonRpcResponse::error(
                            request.id,
                            JsonRpcError::method_not_found("Sampling not yet implemented"),
                        )
                    }
                    _ => {
                        JsonRpcResponse::error(
                            request.id,
                            JsonRpcError::method_not_found(&request.method),
                        )
                    }
                };
                
                if let Err(e) = self.transport.send_response(response).await {
                    error!("Failed to send response: {}", e);
                }
            }
            McpMessage::Notification(notification) => {
                debug!("Received notification: {}", notification.method);
                
                match notification.method.as_str() {
                    "notifications/tools/list_changed" => {
                        info!("Tools list changed, refreshing...");
                        if let Err(e) = self.refresh_tools().await {
                            error!("Failed to refresh tools: {}", e);
                        }
                    }
                    "notifications/resources/list_changed" => {
                        info!("Resources list changed");
                    }
                    _ => {
                        debug!("Unknown notification: {}", notification.method);
                    }
                }
            }
        }
    }
    
    /// Close the connection
    pub async fn close(&self) -> Result<()> {
        self.transport.close().await
    }
}

/// MCP Server Registry - manages multiple MCP server connections
/// 
/// This registry allows managing connections to multiple external MCP servers,
/// enabling CodeTether to use tools from various sources like filesystem servers,
/// database servers, and custom tool servers.
pub struct McpRegistry {
    clients: RwLock<HashMap<String, Arc<McpClient>>>,
    /// Track server capabilities for quick lookup without querying each client
    server_capabilities: RwLock<HashMap<String, ServerCapabilities>>,
    /// Track all discovered tools across servers for efficient discovery
    tool_index: RwLock<HashMap<String, String>>, // tool_name -> server_name
}

impl McpRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            server_capabilities: RwLock::new(HashMap::new()),
            tool_index: RwLock::new(HashMap::new()),
        }
    }
    
    /// Connect to an MCP server and register it with the registry
    pub async fn connect(&self, name: &str, command: &str, args: &[&str]) -> Result<Arc<McpClient>> {
        let transport = Arc::new(ProcessTransport::spawn(command, args).await?);
        let client = Arc::new(McpClient::with_registry(
            transport, 
            Arc::new(McpRegistry::new()), // Each client gets its own registry for now
            Some(name.to_string())
        ));
        
        // Start message receiver
        let client_clone = Arc::clone(&client);
        tokio::spawn(async move {
            client_clone.receive_loop().await;
        });
        
        // Initialize the connection
        let init_result = client.initialize().await?;
        
        // Register the client
        self.register(name, Arc::clone(&client), init_result.capabilities).await;
        
        Ok(client)
    }
    
    /// Register a client with the registry
    pub async fn register(&self, name: &str, client: Arc<McpClient>, capabilities: ServerCapabilities) {
        // Store client
        self.clients.write().await.insert(name.to_string(), client);
        
        // Store capabilities
        self.server_capabilities.write().await.insert(name.to_string(), capabilities);
        
        info!("Registered MCP server '{}' with registry", name);
    }
    
    /// Get a connected client
    pub async fn get(&self, name: &str) -> Option<Arc<McpClient>> {
        self.clients.read().await.get(name).cloned()
    }
    
    /// List all connected servers
    pub async fn list(&self) -> Vec<String> {
        self.clients.read().await.keys().cloned().collect()
    }
    
    /// Get capabilities for a specific server
    pub async fn get_capabilities(&self, name: &str) -> Option<ServerCapabilities> {
        self.server_capabilities.read().await.get(name).cloned()
    }
    
    /// Check if a server has a specific capability
    pub async fn has_capability(&self, name: &str, capability: &str) -> bool {
        let caps = self.server_capabilities.read().await;
        caps.get(name).map(|c| {
            match capability {
                "tools" => c.tools.is_some(),
                "resources" => c.resources.is_some(),
                "prompts" => c.prompts.is_some(),
                "logging" => c.logging.is_some(),
                _ => false,
            }
        }).unwrap_or(false)
    }
    
    /// List servers that have a specific capability
    pub async fn list_by_capability(&self, capability: &str) -> Vec<String> {
        let mut result = Vec::new();
        let caps = self.server_capabilities.read().await;
        
        for (name, caps) in caps.iter() {
            let has_cap = match capability {
                "tools" => caps.tools.is_some(),
                "resources" => caps.resources.is_some(),
                "prompts" => caps.prompts.is_some(),
                "logging" => caps.logging.is_some(),
                _ => false,
            };
            if has_cap {
                result.push(name.clone());
            }
        }
        
        result
    }
    
    /// Disconnect from a server
    pub async fn disconnect(&self, name: &str) -> Result<()> {
        if let Some(client) = self.clients.write().await.remove(name) {
            // Remove capabilities
            self.server_capabilities.write().await.remove(name);
            // Remove from tool index
            let mut tool_index = self.tool_index.write().await;
            tool_index.retain(|_, server| server != name);
            // Close connection
            client.close().await?;
        }
        Ok(())
    }
    
    /// Get all available tools from all servers
    pub async fn all_tools(&self) -> Vec<(String, McpTool)> {
        let mut all_tools = Vec::new();
        
        for (name, client) in self.clients.read().await.iter() {
            for tool in client.tools().await {
                all_tools.push((name.clone(), tool));
            }
        }
        
        all_tools
    }
    
    /// Find a specific tool across all servers
    pub async fn find_tool(&self, tool_name: &str) -> Option<(String, McpTool)> {
        // First check the tool index
        if let Some(server_name) = self.tool_index.read().await.get(tool_name) {
            if let Some(client) = self.get(server_name).await {
                if let Some(tool) = client.tools().await.iter().find(|t| t.name == tool_name) {
                    return Some((server_name.clone(), tool.clone()));
                }
            }
        }
        
        // Fallback: search all clients
        for (name, client) in self.clients.read().await.iter() {
            if let Some(tool) = client.tools().await.iter().find(|t| t.name == tool_name) {
                // Update index
                self.tool_index.write().await.insert(tool_name.to_string(), name.clone());
                return Some((name.clone(), tool.clone()));
            }
        }
        
        None
    }
    
    /// Refresh the tool index from all servers
    pub async fn refresh_tool_index(&self) {
        let mut tool_index = self.tool_index.write().await;
        tool_index.clear();
        
        for (name, client) in self.clients.read().await.iter() {
            for tool in client.tools().await {
                tool_index.insert(tool.name.clone(), name.clone());
            }
        }
        
        info!("Refreshed tool index with {} tools", tool_index.len());
    }
    
    /// Call a tool on a specific server
    pub async fn call_tool(&self, server: &str, tool: &str, arguments: Value) -> Result<CallToolResult> {
        let client = self.get(server).await
            .ok_or_else(|| anyhow::anyhow!("Server not found: {}", server))?;
        client.call_tool(tool, arguments).await
    }
    
    /// Call a tool by name, finding the appropriate server automatically
    pub async fn call_tool_auto(&self, tool_name: &str, arguments: Value) -> Result<CallToolResult> {
        let (server, _) = self.find_tool(tool_name).await
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", tool_name))?;
        self.call_tool(&server, tool_name, arguments).await
    }
}

impl Default for McpRegistry {
    fn default() -> Self {
        Self::new()
    }
}
