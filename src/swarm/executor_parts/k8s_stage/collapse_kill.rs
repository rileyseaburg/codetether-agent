//! Termination of a Kubernetes branch selected by collapse policy.

use super::{collapse_audit, pod_cleanup, state::State};
use crate::swarm::{KillDecision, SubTaskResult, SubTaskStatus};
use crate::tui::swarm_view::SwarmEvent;

pub(super) async fn apply(state: &mut State<'_>, kill: KillDecision) -> Option<SubTaskResult> {
    if state.kill_reasons.contains_key(&kill.subtask_id)
        || !state.active.contains_key(&kill.subtask_id)
    {
        return None;
    }
    pod_cleanup::delete(state, &kill.subtask_id, "collapse kill").await;
    state
        .kill_reasons
        .insert(kill.subtask_id.clone(), kill.reason.clone());
    let elapsed = state
        .active
        .remove(&kill.subtask_id)
        .map(|active| active.started_at.elapsed().as_millis() as u64)
        .unwrap_or(0);
    collapse_audit::kill(state.swarm_id, &kill.subtask_id, &kill.branch, &kill.reason).await;
    state.executor.try_send_event(SwarmEvent::SubTaskUpdate {
        id: kill.subtask_id.clone(),
        name: state
            .names
            .get(&kill.subtask_id)
            .cloned()
            .unwrap_or_else(|| kill.subtask_id.clone()),
        status: SubTaskStatus::Cancelled,
        agent_name: Some(format!("agent-{}", kill.subtask_id)),
    });
    state.executor.try_send_event(SwarmEvent::AgentError {
        subtask_id: kill.subtask_id.clone(),
        error: format!("Cancelled by collapse controller: {}", kill.reason),
    });
    Some(SubTaskResult {
        subtask_id: kill.subtask_id.clone(),
        subagent_id: format!("agent-{}", kill.subtask_id),
        success: false,
        result: format!(
            "\n\n--- Collapse Controller ---\nBranch terminated: {}",
            kill.reason
        ),
        steps: 0,
        tool_calls: 0,
        execution_time_ms: elapsed,
        error: Some(format!("Cancelled by collapse controller: {}", kill.reason)),
        artifacts: Vec::new(),
        retry_count: 0,
    })
}
