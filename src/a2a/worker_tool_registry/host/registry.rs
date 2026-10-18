//! Existing empty-registry construction and insertion.
use super::Native;
use crate::tool::{Tool, ToolRegistry};
use std::sync::Arc;
impl codetether_worker_tools::Registry for Native {
    type Registry = ToolRegistry;
    fn new_registry() -> ToolRegistry {
        ToolRegistry::new()
    }
    fn register(registry: &mut ToolRegistry, tool: Arc<dyn Tool>) {
        registry.register(tool);
    }
}
