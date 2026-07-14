//! Safe identity handling for read-only swarm result caching.

use crate::swarm::{SubTask, SubTaskResult};
use std::collections::HashMap;

pub(super) fn read_only_tasks(tasks: &[SubTask]) -> HashMap<String, SubTask> {
    tasks
        .iter()
        .filter(|task| task.is_read_only())
        .map(|task| (task.id.clone(), task.clone()))
        .collect()
}

pub(super) fn retarget(mut result: SubTaskResult, task: &SubTask) -> SubTaskResult {
    result.subtask_id = task.id.clone();
    result.subagent_id = format!("cached-{}", task.id);
    result
}

#[cfg(test)]
mod tests {
    use super::read_only_tasks;
    use crate::swarm::SubTask;

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
