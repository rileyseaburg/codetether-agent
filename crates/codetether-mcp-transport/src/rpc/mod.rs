//! Request correlation and pending response channels for one MCP connection.
//!
//! The caller drives incoming messages and delivers responses. Initialization,
//! remote tool policy and server registration stay with the client adapter.
use crate::Transport;
use codetether_mcp_core::types::{JsonRpcResponse, RequestId};
use std::collections::HashMap;
use std::sync::{Arc, atomic::AtomicI64};
use tokio::sync::{RwLock, oneshot};
mod notify;
mod request;
mod response;
/// Per-connection request IDs and pending response channels.
///
/// # Examples
/// ```
/// use codetether_mcp_transport::{NullTransport, rpc::RpcConnection};
/// let connection = RpcConnection::new(std::sync::Arc::new(NullTransport::new()));
/// ```
pub struct RpcConnection {
    transport: Arc<dyn Transport>,
    pending_requests: RwLock<HashMap<RequestId, oneshot::Sender<JsonRpcResponse>>>,
    request_id: AtomicI64,
}
impl RpcConnection {
    /// Create an empty connection whose first request ID is one.
    pub fn new(transport: Arc<dyn Transport>) -> Self {
        Self {
            transport,
            pending_requests: RwLock::new(HashMap::new()),
            request_id: AtomicI64::new(1),
        }
    }
    /// Borrow the transport without changing its Arc ownership count.
    pub fn transport(&self) -> &dyn Transport {
        self.transport.as_ref()
    }
}
