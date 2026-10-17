//! Route requests and update notification state.
use super::*;
impl McpServer {
    /// Handle a JSON-RPC request
    pub(super) async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        tracing::debug!(target: "codetether_agent::mcp::server",
            "Handling request: {} (id: {:?})",
            request.method, request.id
        );

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
            method => super::super::approval::handle(method, request.params).await,
        };

        match result {
            Ok(value) => JsonRpcResponse::success(request.id, value),
            Err(error) => JsonRpcResponse::error(request.id, error),
        }
    }

    /// Handle a notification
    pub(super) async fn handle_notification(&self, notification: JsonRpcNotification) {
        tracing::debug!(target: "codetether_agent::mcp::server", "Handling notification: {}", notification.method);

        match notification.method.as_str() {
            "notifications/initialized" => {
                *self.initialized.write().await = true;
                tracing::info!(target: "codetether_agent::mcp::server", "MCP client initialized");
            }
            "notifications/cancelled" => {
                // Handle cancellation
            }
            _ => {
                tracing::debug!(target: "codetether_agent::mcp::server", "Unknown notification: {}", notification.method);
            }
        }
    }
}
