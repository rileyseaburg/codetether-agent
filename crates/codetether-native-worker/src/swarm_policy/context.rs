//! Worker envelope retained across swarm execution and session continuation.
use codetether_a2a_worker_core::execution_policy::AutoApprove;
use codetether_bus_core::AgentBus;
use std::sync::Arc;
pub(super) struct Context<'a, S> {
    pub session: &'a mut S,
    pub prompt: &'a str,
    pub auto_approve: AutoApprove,
    pub model_tier: Option<&'a str>,
    pub explicit_model: Option<String>,
    pub metadata: &'a serde_json::Map<String, serde_json::Value>,
    pub complexity_hint: Option<&'a str>,
    pub worker_personality: Option<&'a str>,
    pub target_agent_name: Option<&'a str>,
    pub bus: Option<&'a Arc<AgentBus>>,
    pub output: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}
