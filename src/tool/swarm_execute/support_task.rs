use crate::swarm::SubTask;

pub(super) fn build(
    name: &str,
    instruction: &str,
    specialty: Option<&str>,
    needs_worktree: Option<bool>,
) -> SubTask {
    let mut task = SubTask::new(name, instruction);
    task.specialty = specialty.map(String::from);
    if let Some(needs) = needs_worktree {
        task = task.with_needs_worktree(needs);
    }
    task
}
