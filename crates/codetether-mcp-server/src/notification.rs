//! Initialization and cancellation notification handling.
use crate::Host;
use codetether_mcp_core::types::JsonRpcNotification;
/// Apply one incoming notification to the server state.
/// # Arguments
/// * `host` - Existing server initialization flag.
/// * `notification` - Incoming notification.
/// # Returns
/// Updates initialization when requested; other notifications retain their behavior.
/// # Examples
/// ```no_run
/// async fn receive(host: &impl codetether_mcp_server::Host, notification: codetether_mcp_core::types::JsonRpcNotification) {
///     codetether_mcp_server::notification(host, notification).await;
/// }
/// ```
pub async fn notification(host: &impl Host, notification: JsonRpcNotification) {
    tracing::debug!(target: "codetether_agent::mcp::server", "Handling notification: {}", notification.method);

    match notification.method.as_str() {
        "notifications/initialized" => {
            *host.initialized().write().await = true;
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
