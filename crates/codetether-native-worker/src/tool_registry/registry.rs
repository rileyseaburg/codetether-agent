//! Existing empty-registry construction and insertion.
use super::Native;
use codetether_application_registry::{Application, ToolRegistry};
use codetether_tool_core::Tool;
use std::sync::Arc;
impl<A: Application, W> codetether_worker_tools::Registry for Native<A, W> {
    type Registry = ToolRegistry<A>;
    fn new_registry() -> ToolRegistry<A> {
        ToolRegistry::<A>::new()
    }
    fn register(registry: &mut ToolRegistry<A>, tool: Arc<dyn Tool>) {
        registry.register(tool);
    }
}
