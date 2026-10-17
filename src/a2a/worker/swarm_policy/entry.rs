//! Adapt the stable worker call to a structured execution envelope.

use super::context::Context;
use crate::{
    a2a::worker::AutoApprove,
    bus::AgentBus,
    session::{Session, SessionResult},
};
use std::sync::Arc;

pub(in crate::a2a::worker) async fn execute_swarm_with_policy(
    session: &mut Session,
    prompt: &str,
    auto_approve: AutoApprove,
    model_tier: Option<&str>,
    explicit_model: Option<String>,
    metadata: &serde_json::Map<String, serde_json::Value>,
    complexity_hint: Option<&str>,
    worker_personality: Option<&str>,
    target_agent_name: Option<&str>,
    bus: Option<&Arc<AgentBus>>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> anyhow::Result<(SessionResult, bool)> {
    super::execute::run(Context {
        session,
        prompt,
        auto_approve,
        model_tier,
        explicit_model,
        metadata,
        complexity_hint,
        worker_personality,
        target_agent_name,
        bus,
        output: output_callback,
    })
    .await
}
