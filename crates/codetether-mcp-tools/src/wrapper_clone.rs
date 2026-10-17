//! Clone shared ownership without requiring the host client itself to be Clone.
use crate::McpToolWrapper;
impl<C> Clone for McpToolWrapper<C> {
    fn clone(&self) -> Self {
        Self {
            client: std::sync::Arc::clone(&self.client),
            tool: self.tool.clone(),
            id: self.id.clone(),
        }
    }
}
