//! Native binding for standard shared worker-session execution.
mod host;
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
    codetether_a2a_worker_core::session_protocol::policy::execute(
        &mut host::Native { session },
        prompt,
        auto_approve,
        model_tier,
        task_metadata,
        output_callback,
    )
    .await
}
