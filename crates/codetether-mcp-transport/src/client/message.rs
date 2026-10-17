//! Deliver responses and handle peer requests and notifications.
use super::*;
impl RemoteClient {
    /// Handle an incoming message
    pub(super) async fn handle_message(&self, message: McpMessage) {
        match message {
            McpMessage::Response(response) => {
                // Find and notify the waiting request
                self.connection.deliver(response).await;
            }
            McpMessage::Request(request) => {
                // Server is making a request to us (e.g., sampling)
                tracing::debug!(target: "codetether_agent::mcp::client", "Received request from server: {}", request.method);

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
                    tracing::error!(target: "codetether_agent::mcp::client", "Failed to send response: {}", e);
                }
            }
            McpMessage::Notification(notification) => {
                tracing::debug!(target: "codetether_agent::mcp::client", "Received notification: {}", notification.method);

                match notification.method.as_str() {
                    "notifications/tools/list_changed" => {
                        tracing::info!(target: "codetether_agent::mcp::client", "Tools list changed, refreshing...");
                        if let Err(e) = self.refresh_tools().await {
                            tracing::error!(target: "codetether_agent::mcp::client", "Failed to refresh tools: {}", e);
                        }
                    }
                    "notifications/resources/list_changed" => {
                        tracing::info!(target: "codetether_agent::mcp::client", "Resources list changed");
                    }
                    _ => {
                        tracing::debug!(target: "codetether_agent::mcp::client", "Unknown notification: {}", notification.method);
                    }
                }
            }
        }
    }
}
