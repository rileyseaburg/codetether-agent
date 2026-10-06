//! Separate human-facing goal and task projections of the shared journal.

use crate::session::tasks::{SessionTaskStatus, TaskState};
use std::fmt::Write;

/// Render the objective and its governance, without the session work items.
///
/// # Arguments
/// * `state` — Materialized journal state.
/// # Returns
/// Goal text, or `None` when no goal exists.
/// # Examples
/// ```
/// use codetether_agent::session::tasks::{goal_block, TaskState};
/// assert!(goal_block(&TaskState::default()).is_none());
/// ```
pub fn goal_block(state: &TaskState) -> Option<String> {
    let goal = state.goal.as_ref()?;
    let mut output = String::from("## Session goal — overall objective\n");
    super::goal::append(&mut output, goal);
    Some(output)
}

/// Render all work items, including closed ones, without embedding the goal.
///
/// # Arguments
/// * `state` — Materialized journal state.
/// # Returns
/// Task statuses and contents, or an explicit empty-list message.
/// # Examples
/// ```
/// use codetether_agent::session::tasks::{task_block, TaskState};
/// assert!(task_block(&TaskState::default()).contains("No session tasks"));
/// ```
pub fn task_block(state: &TaskState) -> String {
    let mut output = String::from("## Session tasks — work items, not the goal\n");
    if state.tasks.is_empty() {
        output.push_str("No session tasks.\n");
    }
    for task in state.tasks.values() {
        let status = match task.status {
            SessionTaskStatus::Pending => "pending",
            SessionTaskStatus::InProgress => "in_progress",
            SessionTaskStatus::Done => "done",
            SessionTaskStatus::Blocked => "blocked",
            SessionTaskStatus::Cancelled => "cancelled",
        };
        let _ = writeln!(output, "[{status}] {}: {}", task.id, task.content);
    }
    output
}
