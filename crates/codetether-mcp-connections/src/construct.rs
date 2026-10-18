//! Empty connection registry construction.
use super::{ConnectionClient, Connections};
use tokio::sync::RwLock;
impl<C: ConnectionClient> Connections<C> {
    /// Create an empty registry.
    /// # Examples
    /// ```
    /// fn empty<C: codetether_mcp_connections::ConnectionClient>() {
    ///     let _ = codetether_mcp_connections::Connections::<C>::new();
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
}
impl<C> Default for Connections<C> {
    fn default() -> Self {
        Self {
            connections: RwLock::new(Vec::new()),
        }
    }
}
