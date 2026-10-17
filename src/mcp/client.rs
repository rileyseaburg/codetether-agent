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
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

#[path = "client_call_tool.rs"]
mod call_tool;
#[path = "client_subprocess.rs"]
mod subprocess;

/// MCP Client for connecting to external servers
pub struct McpClient {
    connection: codetether_mcp_transport::rpc::RpcConnection,
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
        Self::connect_subprocess_with_approval(command, args, None).await
    }

    /// Create a new MCP client with custom transport
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self {
            connection: codetether_mcp_transport::rpc::RpcConnection::new(transport),
            server_info: RwLock::new(None),
            server_capabilities: RwLock::new(None),
            available_tools: RwLock::new(Vec::new()),
            registry: Arc::new(McpRegistry::new()),
            server_name: RwLock::new(None),
        }
    }

    /// Create a new MCP client with a shared registry for multi-server management
    pub fn with_registry(
        transport: Arc<dyn Transport>,
        registry: Arc<McpRegistry>,
        name: Option<String>,
    ) -> Self {
        Self {
            connection: codetether_mcp_transport::rpc::RpcConnection::new(transport),
            server_info: RwLock::new(None),
            server_capabilities: RwLock::new(None),
            available_tools: RwLock::new(Vec::new()),
            registry,
            server_name: RwLock::new(name),
        }
    }

    /// Initialize the connection with the server
    pub async fn initialize(&self) -> Result<InitializeResult> {
        let result = self
            .connection
            .initialize(env!("CARGO_PKG_VERSION"))
            .await?;

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
            "resources" => caps
                .as_ref()
                .map(|c| c.resources.is_some())
                .unwrap_or(false),
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
        let tools = self.connection.list_tools().await?;

        *self.available_tools.write().await = tools.clone();

        info!("Loaded {} tools from MCP server", tools.len());

        Ok(tools)
    }

    /// Get available tools
    pub async fn tools(&self) -> Vec<McpTool> {
        self.available_tools.read().await.clone()
    }

    /// Call a tool
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        self.call_tool_checked(name, arguments).await
    }
    /// List remote resources.
    pub async fn list_resources(&self) -> Result<Vec<McpResource>> {
        self.connection.list_resources().await
    }
    /// Read one remote resource.
    pub async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult> {
        self.connection.read_resource(uri).await
    }
    /// List remote prompt templates.
    pub async fn list_prompts(&self) -> Result<Vec<McpPrompt>> {
        self.connection.list_prompts().await
    }
    /// Retrieve a prompt template with arguments.
    pub async fn get_prompt(&self, name: &str, arguments: Value) -> Result<GetPromptResult> {
        self.connection.get_prompt(name, arguments).await
    }
    async fn notify(&self, method: &str, params: Option<Value>) -> Result<()> {
        self.connection.notify(method, params).await
    }

    /// Message receive loop
    async fn receive_loop(&self) {
        loop {
            match self.connection.transport().receive().await {
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
                self.connection.deliver(response).await;
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
                    _ => JsonRpcResponse::error(
                        request.id,
                        JsonRpcError::method_not_found(&request.method),
                    ),
                };

                if let Err(e) = self.connection.transport().send_response(response).await {
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
        self.connection.transport().close().await
    }
}
/// MCP registry with application-authorized connection setup.
pub struct McpRegistry {
    inner: codetether_mcp_core::registry::ServerRegistry<McpClient>,
}

#[path = "client_registry.rs"]
mod registry;
#[path = "client_registry_client.rs"]
mod registry_client;
