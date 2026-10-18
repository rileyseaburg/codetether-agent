//! Verify unfinished parent goals and steering through the standard worker loop.
use super::{Application, Session, SessionResult, ToolRegistry};
use codetether_a2a_worker_core::execution_policy::AutoApprove;
use codetether_native_session_runtime::continuation::{self, Continuation};
use std::sync::Arc;
pub(super) async fn after<S, T, W>(
    session: &mut S,
    result: (SessionResult, bool),
    steered: bool,
    approval: AutoApprove,
    tier: Option<&str>,
    metadata: &serde_json::Map<String, serde_json::Value>,
    output: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> anyhow::Result<(SessionResult, bool)>
where
    S: Session,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>>,
{
    let next = continuation::after_message(session).await?;
    if !steered && matches!(next, Continuation::Finish) {
        return Ok(result);
    }
    let outcome = crate::session_policy::execute::<S, T, W>(session,
        "Review the saved swarm results against the existing objective and latest human steering. Continue any unfinished work; preserve completed work. Complete an active goal only through its normal verified completion workflow.",
        approval, tier, metadata, output,
    ).await?;
    Ok((outcome.result, !outcome.budget_exhausted))
}
