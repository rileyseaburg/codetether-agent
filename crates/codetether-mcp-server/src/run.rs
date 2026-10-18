//! Sequential receive, dispatch and response delivery.
use crate::Host;
use anyhow::Result;
use codetether_mcp_transport::McpMessage;
/// Serve messages until the existing transport closes.
/// # Arguments
/// * `host` - Server transport, catalog and application policy bindings.
/// # Returns
/// Success after transport closure.
/// # Errors
/// Returns transport receive or response-send failures.
/// # Examples
/// ```no_run
/// async fn serve(host: &impl codetether_mcp_server::Host) -> anyhow::Result<()> {
///     codetether_mcp_server::run(host).await
/// }
/// ```
pub async fn run(host: &impl Host) -> Result<()> {
    tracing::info!(target: "codetether_agent::mcp::server", "Starting MCP server...");

    // Register tools before starting
    host.setup().await;

    loop {
        match host.transport().receive().await? {
            Some(McpMessage::Request(request)) => {
                let response = crate::request(host, request).await;
                host.transport().send_response(response).await?;
            }
            Some(McpMessage::Notification(notification)) => {
                crate::notification(host, notification).await;
            }
            Some(McpMessage::Response(response)) => {
                // We received a response (shouldn't happen in server mode)
                tracing::warn!(target: "codetether_agent::mcp::server", "Unexpected response received: {:?}", response.id);
            }
            None => {
                tracing::info!(target: "codetether_agent::mcp::server", "Transport closed, shutting down MCP server");
                break;
            }
        }
    }

    Ok(())
}
