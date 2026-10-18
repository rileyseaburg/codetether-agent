//! Named MCP connections, discovery and readable tool-call output.
//!
//! Connections retains clients implementing ConnectionClient. The caller owns
//! subprocess authorization; this crate preserves connection order and routes
//! calls through RegistryClient's existing authorization implementation.
mod call;
mod client;
mod connect;
mod connection;
mod construct;
mod servers;
mod summary;
mod tools;
pub use client::ConnectionClient;
use connection::Connection;
pub use connection::ServerSummary;
use tokio::sync::RwLock;
/// Ordered named MCP connections supplied by an authorized client adapter.
/// # Examples
/// ```
/// fn empty<C: codetether_mcp_connections::ConnectionClient>() {
///     let _ = codetether_mcp_connections::Connections::<C>::new();
/// }
/// ```
pub struct Connections<C> {
    connections: RwLock<Vec<Connection<C>>>,
}
