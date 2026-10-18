//! Delegate application-specific workflow construction without changing inputs.
use super::Native;
use codetether_native_providers::Provider;
use codetether_tool_core::Tool;
use codetether_worker_tools::Workflow;
use std::sync::Arc;
impl<A, W: Workflow> Workflow for Native<A, W> {
    fn mcp_bridge() -> Arc<dyn Tool> {
        W::mcp_bridge()
    }
    fn rlm(provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool> {
        W::rlm(provider, model)
    }
    fn ralph(provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool> {
        W::ralph(provider, model)
    }
    fn go(callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>) -> Arc<dyn Tool> {
        W::go(callback)
    }
}
