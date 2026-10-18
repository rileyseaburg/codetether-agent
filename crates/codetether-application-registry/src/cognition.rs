//! Shared cognition registry integration.
use super::{Application, ToolRegistry};
use codetether_tool_core::Tool;
use std::sync::Arc;
impl<H: Application> codetether_cognition_core::execution::dispatch::Registry for ToolRegistry<H> {
    fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        ToolRegistry::get(self, id)
    }
}
