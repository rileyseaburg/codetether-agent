//! Generic MCP registry construct operations.
use super::*;
impl<C: RegistryClient> ServerRegistry<C> {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            server_capabilities: RwLock::new(HashMap::new()),
            tool_index: RwLock::new(HashMap::new()),
        }
    }
}

impl<C: RegistryClient> Default for ServerRegistry<C> {
    fn default() -> Self {
        Self::new()
    }
}
