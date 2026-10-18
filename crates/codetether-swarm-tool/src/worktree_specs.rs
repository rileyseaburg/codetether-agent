//! Projection from invocation tasks to worktree allocation inputs.
use super::task_input::TaskInput;

/// Task intent name, instruction, specialty, and explicit worktree flag.
pub type WorktreeSpec = (String, String, Option<String>, Option<bool>);

/// Prepare allocation specifications without changing task order.
/// # Arguments
/// * `tasks` — Normalized task entries.
/// # Returns
/// One specification per task.
/// # Examples
/// ```
/// assert!(codetether_swarm_tool::worktree_specs::from_tasks(&[]).is_empty());
/// ```
pub fn from_tasks(tasks: &[TaskInput]) -> Vec<WorktreeSpec> {
    tasks
        .iter()
        .map(|task| {
            (
                task.intent_name().to_string(),
                task.instruction.clone(),
                task.specialty.clone(),
                task.needs_worktree,
            )
        })
        .collect()
}
