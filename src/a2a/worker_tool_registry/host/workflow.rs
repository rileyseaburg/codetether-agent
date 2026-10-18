//! Application model and background execution tool specializations.
pub(crate) struct Native;
use crate::provider::Provider;
use crate::tool::{Tool, go, mcp_bridge, ralph, rlm};
use std::sync::Arc;
impl codetether_worker_tools::Workflow for Native {
    fn mcp_bridge() -> Arc<dyn Tool> {
        Arc::new(mcp_bridge::McpBridgeTool::new())
    }
    fn rlm(provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool> {
        Arc::new(rlm::RlmTool::new(
            provider,
            model,
            crate::rlm::RlmConfig::default(),
        ))
    }
    fn ralph(provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool> {
        Arc::new(ralph::RalphTool::with_provider(provider, model))
    }
    fn go(callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>) -> Arc<dyn Tool> {
        Arc::new(match callback {
            Some(callback) => go::GoTool::with_callback(callback),
            None => go::GoTool::new(),
        })
    }
}
