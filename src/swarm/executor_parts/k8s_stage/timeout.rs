//! Timeout detection for active Kubernetes sub-agents.

use super::{
    failure, pod_cleanup,
    state::{ActiveBranch, State},
};
use crate::swarm::SubTaskResult;
use std::time::Duration;

pub(super) async fn check(
    state: &mut State<'_>,
    id: &str,
    active: &ActiveBranch,
) -> Option<SubTaskResult> {
    let limit = state.executor.config.subagent_timeout_secs;
    if active.started_at.elapsed() <= Duration::from_secs(limit) {
        return None;
    }
    let reason = format!("Timed out after {limit}s in Kubernetes pod");
    state.kill_reasons.insert(id.into(), reason.clone());
    pod_cleanup::delete(state, id, "timeout").await;
    state.active.remove(id);
    Some(failure::result(
        id,
        reason,
        active.started_at.elapsed().as_millis() as u64,
    ))
}
