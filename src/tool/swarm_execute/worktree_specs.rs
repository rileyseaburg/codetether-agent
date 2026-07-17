use super::task_input::TaskInput;

pub(super) type WorktreeSpec = (String, String, Option<String>, Option<bool>);

pub(super) fn from_tasks(tasks: &[TaskInput]) -> Vec<WorktreeSpec> {
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
