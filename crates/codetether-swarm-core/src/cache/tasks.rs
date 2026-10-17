//! Safe identity handling for read-only swarm result caching.

use crate::{SubTask, SubTaskResult};
use std::collections::HashMap;

/// Select tasks eligible for read-only result caching.
///
/// # Arguments
/// * `tasks` - Tasks to inspect using their existing intent classification.
///
/// # Returns
/// Cloned read-only tasks indexed by ID.
///
/// # Examples
/// ```
/// assert!(codetether_swarm_core::cache::read_only_tasks(&[]).is_empty());
/// ```
pub fn read_only_tasks(tasks: &[SubTask]) -> HashMap<String, SubTask> {
    tasks
        .iter()
        .filter(|task| task.is_read_only())
        .map(|task| (task.id.clone(), task.clone()))
        .collect()
}

/// Retarget a cached result to the current task identity.
///
/// # Arguments
/// * `result` - Cached result to update.
/// * `task` - Current task supplying its ID.
///
/// # Returns
/// The result with current task and cached-agent identifiers.
///
/// # Examples
/// ```
/// # fn example(result: codetether_swarm_core::SubTaskResult) {
/// let task = codetether_swarm_core::SubTask::new("inspect", "read source");
/// let cached = codetether_swarm_core::cache::retarget(result, &task);
/// assert_eq!(cached.subtask_id, task.id);
/// # }
/// ```
pub fn retarget(mut result: SubTaskResult, task: &SubTask) -> SubTaskResult {
    result.subtask_id = task.id.clone();
    result.subagent_id = format!("cached-{}", task.id);
    result
}

#[cfg(test)]
mod tests {
    use super::read_only_tasks;
    use crate::SubTask;

    #[test]
    fn mutating_tasks_are_never_cacheable() {
        let mut review = SubTask::new("review", "inspect");
        review.specialty = Some("reviewer".into());
        let edit = SubTask::new("edit", "edit the file");
        let tasks = read_only_tasks(&[review.clone(), edit]);
        assert!(tasks.contains_key(&review.id));
        assert_eq!(tasks.len(), 1);
    }
}
