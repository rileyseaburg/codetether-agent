//! Shared swarm registry integration.
use super::{Application, ToolRegistry};
impl<H: Application> codetether_swarm_core::tool_policy::Registry for ToolRegistry<H> {
    fn list(&self) -> Vec<&str> {
        ToolRegistry::list(self)
    }
    fn unregister(&mut self, name: &str) {
        ToolRegistry::unregister(self, name);
    }
}
