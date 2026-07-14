//! Expected file-change outcomes for isolated swarm subtasks.

use crate::swarm::SubTask;
use std::collections::HashMap;

pub(super) fn for_tasks(tasks: &[SubTask]) -> HashMap<String, bool> {
    tasks
        .iter()
        .map(|task| (task.id.clone(), task.expects_file_changes()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::for_tasks;
    use crate::swarm::SubTask;

    #[test]
    fn verification_does_not_require_source_changes() {
        let task = SubTask::new("verify", "Run focused tests");
        assert_eq!(for_tasks(&[task]).values().copied().next(), Some(false));
    }
}
