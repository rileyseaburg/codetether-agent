//! Application of local collapse-controller kill decisions.

use super::{collapse_audit, collapse_events, state::State};
use crate::swarm::collapse_controller::KillDecision;

pub(super) async fn apply(state: &mut State<'_>, kill: KillDecision) {
    if state.kill_reasons.contains_key(&kill.subtask_id) {
        return;
    }
    let Some(handle) = state.aborts.remove(&kill.subtask_id) else {
        return;
    };
    handle.abort();
    state.active_worktrees.remove(&kill.subtask_id);
    state
        .kill_reasons
        .insert(kill.subtask_id.clone(), kill.reason.clone());
    tracing::warn!(subtask_id = %kill.subtask_id, branch = %kill.branch,
        reason = %kill.reason, "Collapse controller killed branch");
    collapse_events::killed(state, &kill.subtask_id, &kill.reason);
    collapse_audit::kill(state.swarm_id, &kill.subtask_id, &kill.branch, &kill.reason).await;
}
