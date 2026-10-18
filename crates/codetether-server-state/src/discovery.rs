//! Native registry access for the shared discovery routes.
use crate::AppState;
impl<R: Send + Sync + 'static> codetether_tool_discovery::HttpState for AppState<R> {
    fn tool_registry(&self) -> &codetether_tool_discovery::ToolRegistry {
        &self.inner.tool_registry
    }
}
