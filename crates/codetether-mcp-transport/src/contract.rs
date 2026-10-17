//! Asynchronous JSON-RPC transport operations.
use crate::*;

/// Transport trait for MCP communication.
///
/// # Implementors
/// [StdioTransport], [NullTransport], [SseTransport] and [ProcessTransport].
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a JSON-RPC request
    async fn send_request(&self, request: JsonRpcRequest) -> Result<()>;

    /// Send a JSON-RPC response
    async fn send_response(&self, response: JsonRpcResponse) -> Result<()>;

    /// Send a JSON-RPC notification
    async fn send_notification(&self, notification: JsonRpcNotification) -> Result<()>;

    /// Receive incoming messages
    async fn receive(&self) -> Result<Option<McpMessage>>;

    /// Close the transport
    async fn close(&self) -> Result<()>;
}
