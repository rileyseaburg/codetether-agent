//! Protocol session state for one remote MCP peer.
//!
//! [RemoteClient] caches initialized metadata and tools, handles peer messages,
//! and exposes resource and prompt operations. Callers authorize outgoing tools
//! and subprocess creation and supply their identity version at initialization.
use crate::{McpMessage, Transport, rpc::RpcConnection};
use anyhow::Result;
use codetether_mcp_core::types::*;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
mod capabilities;
mod connection;
mod identity;
mod initialize;
mod message;
mod prompts;
mod receive;
mod resources;
mod tools;
/// One remote peer's connection and initialized metadata.
///
/// # Examples
/// ```
/// use codetether_mcp_transport::{client::RemoteClient, NullTransport};
/// let peer = RemoteClient::new(std::sync::Arc::new(NullTransport::new()), None);
/// ```
pub struct RemoteClient {
    connection: RpcConnection,
    server_info: RwLock<Option<ServerInfo>>,
    server_capabilities: RwLock<Option<ServerCapabilities>>,
    available_tools: RwLock<Vec<McpTool>>,
    server_name: RwLock<Option<String>>,
}
impl RemoteClient {
    /// Construct a session without starting transport reception or initialization.
    ///
    /// # Arguments
    /// `transport` is the connection; `name` is an optional registry-facing label.
    ///
    /// # Returns
    /// An uninitialized client with empty metadata and tool caches.
    ///
    /// # Examples
    /// ```
    /// use codetether_mcp_transport::{client::RemoteClient, NullTransport};
    /// let client = RemoteClient::new(std::sync::Arc::new(NullTransport::new()), None);
    /// ```
    pub fn new(transport: Arc<dyn Transport>, name: Option<String>) -> Self {
        Self {
            connection: RpcConnection::new(transport),
            server_info: RwLock::new(None),
            server_capabilities: RwLock::new(None),
            available_tools: RwLock::new(Vec::new()),
            server_name: RwLock::new(name),
        }
    }
}
