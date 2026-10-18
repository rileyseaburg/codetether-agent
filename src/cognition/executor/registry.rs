//! Bind native registry lookup to shared cognition dispatch.
use crate::tool::ToolRegistry;
use codetether_tool_core::Tool;
use std::sync::Arc;
impl codetether_cognition_core::execution::dispatch::Registry for ToolRegistry {
    fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        ToolRegistry::get(self, id)
    }
}
