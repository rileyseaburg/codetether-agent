//! Borrow the tool map for host execution under the existing read lock.
use super::*;
impl ServerCatalog {
    /// Borrow registered tool handlers without changing lock ownership.
    pub fn tools(&self) -> &RwLock<HashMap<String, McpToolHandler>> {
        &self.tools
    }
}
