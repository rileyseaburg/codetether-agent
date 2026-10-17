//! Expected file-change outcomes for isolated swarm subtasks.

use crate::SubTask;
use std::collections::HashMap;

/// Collect expected file-mutation flags for each task.
///
/// # Arguments
/// * `tasks` - Tasks whose mutation intent should be recorded.
///
/// # Returns
/// Task IDs mapped to their existing expected-change predicate.
///
/// # Examples
/// ```
/// assert!(codetether_swarm_core::run_policy::expected_changes(&[]).is_empty());
/// ```
pub fn for_tasks(tasks: &[SubTask]) -> HashMap<String, bool> {
    tasks
        .iter()
        .map(|task| (task.id.clone(), task.expects_file_changes()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::for_tasks;
    use crate::SubTask;

    #[test]
    fn verification_does_not_require_source_changes() {
        let task = SubTask::new("verify", "Run focused tests");
        assert_eq!(for_tasks(&[task]).values().copied().next(), Some(false));
    }
}
