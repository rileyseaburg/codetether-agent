//! Native registry access for the shared discovery routes.
impl codetether_tool_discovery::HttpState for super::AppState {
    fn tool_registry(&self) -> &codetether_tool_discovery::ToolRegistry {
        &self.tool_registry
    }
}
