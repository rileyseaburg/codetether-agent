use crate::swarm::subtask::SubTask;

type TaskSpec = (String, String, Option<String>, Option<bool>);

pub(super) fn prepare(tasks: &[TaskSpec]) -> (Vec<(String, SubTask)>, Vec<bool>, Vec<bool>) {
    let mut prepared = Vec::with_capacity(tasks.len());
    let mut expects_changes = Vec::with_capacity(tasks.len());
    let mut verification = Vec::with_capacity(tasks.len());
    for (name, instruction, specialty, needs_worktree) in tasks {
        let task = crate::tool::swarm_execute::support_task::build(
            name,
            instruction,
            specialty.as_deref(),
            *needs_worktree,
        );
        expects_changes.push(task.expects_file_changes());
        verification.push(task.is_verification());
        prepared.push((name.clone(), task));
    }
    (prepared, expects_changes, verification)
}
