//! Human-readable swarm progress formatting.

use crate::tui::swarm_view::SwarmEvent;

pub(super) fn format_swarm_event_for_output(event: &SwarmEvent) -> Option<String> {
    match event {
        SwarmEvent::Started {
            task,
            total_subtasks,
        } => Some(format!(
            "[swarm] started task={} planned_subtasks={}",
            task, total_subtasks
        )),
        SwarmEvent::StageComplete {
            stage,
            completed,
            failed,
        } => Some(format!(
            "[swarm] stage={} completed={} failed={}",
            stage, completed, failed
        )),
        SwarmEvent::SubTaskUpdate { id, status, .. } => Some(format!(
            "[swarm] subtask id={} status={}",
            &id.chars().take(8).collect::<String>(),
            format!("{status:?}").to_ascii_lowercase()
        )),
        SwarmEvent::AgentToolCall {
            subtask_id,
            tool_name,
        } => Some(format!(
            "[swarm] subtask id={} tool={}",
            &subtask_id.chars().take(8).collect::<String>(),
            tool_name
        )),
        SwarmEvent::AgentError { subtask_id, error } => Some(format!(
            "[swarm] subtask id={} error={}",
            &subtask_id.chars().take(8).collect::<String>(),
            error
        )),
        SwarmEvent::Complete { success, stats } => Some(format!(
            "[swarm] complete success={} subtasks={} speedup={:.2}",
            success,
            stats.subagents_completed + stats.subagents_failed,
            stats.speedup_factor
        )),
        SwarmEvent::Error(error) => Some(format!("[swarm] error message={}", error)),
        _ => None,
    }
}
