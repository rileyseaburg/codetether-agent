//! Native binding for standard shared worker-session execution.
mod source;
use super::AutoApprove;
use crate::session::Session;
pub(super) use codetether_a2a_worker_core::session_protocol::policy::PolicySessionResult;
use std::sync::Arc;
pub(super) async fn execute_session_with_policy(
    session: &mut Session,
    prompt: &str,
    auto_approve: AutoApprove,
    model_tier: Option<&str>,
    task_metadata: &serde_json::Map<String, serde_json::Value>,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> anyhow::Result<PolicySessionResult> {
    codetether_native_worker::session_policy::execute::<
        _,
        crate::tool::registry_lifecycle::Native,
        crate::a2a::worker_tool_registry::Native,
    >(
        session,
        prompt,
        auto_approve,
        model_tier,
        task_metadata,
        output_callback,
    )
    .await
}
