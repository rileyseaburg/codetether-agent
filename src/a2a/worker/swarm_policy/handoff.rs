//! Verify unfinished parent goals and steering through the shared session loop.

use crate::a2a::worker::{AutoApprove, execute_session_with_policy};
use crate::session::{
    Session, SessionResult,
    runtime::continuation::{self, Continuation},
};
use std::sync::Arc;

pub(super) async fn after(
    session: &mut Session,
    result: (SessionResult, bool),
    steered: bool,
    approval: AutoApprove,
    tier: Option<&str>,
    metadata: &serde_json::Map<String, serde_json::Value>,
    output: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> anyhow::Result<(SessionResult, bool)> {
    let next = continuation::after_message(session).await?;
    if !steered && matches!(next, Continuation::Finish) {
        return Ok(result);
    }
    let outcome = execute_session_with_policy(session,
        "Review the saved swarm results against the existing objective and latest human steering. Continue any unfinished work; preserve completed work. Complete an active goal only through its normal verified completion workflow.",
        approval, tier, metadata, output,
    ).await?;
    Ok((outcome.result, !outcome.budget_exhausted))
}
