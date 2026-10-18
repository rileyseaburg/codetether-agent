//! Native runtime policy and parent-goal accounting for shared sub-agents.
use crate::tool::{Tool, ToolRegistry};
use serde_json::Value;
use std::sync::Arc;
pub(super) struct Native;
impl codetether_swarm_agent::Host for Native {
    type Registry = ToolRegistry;
    fn get(registry: &ToolRegistry, name: &str) -> Option<Arc<dyn Tool>> {
        registry.get(name)
    }
}
impl codetether_provider_step::controlled::Accounting for Native {
    async fn record(
        session_id: &str,
        tokens: usize,
        elapsed: std::time::Duration,
    ) -> anyhow::Result<()> {
        crate::session::runtime::goals::record_usage(session_id, tokens, elapsed).await
    }
}
impl codetether_swarm_core::agent_loop::tools::input::Policy for Native {
    async fn denial(name: &str, args: &Value) -> Option<String> {
        crate::swarm::tool_policy::runtime_denial(name, args).await
    }
}
