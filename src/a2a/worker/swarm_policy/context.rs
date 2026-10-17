//! Worker envelope retained across swarm execution and session continuation.

use crate::{a2a::worker::AutoApprove, bus::AgentBus, session::Session};
use std::sync::Arc;

pub(super) struct Context<'a> {
    pub session: &'a mut Session,
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
