//! Repository-aware system prompt for direct swarm workers.

use crate::swarm::tool_policy::{SystemPromptInput, system_prompt};
use std::path::Path;

use super::task_input::TaskInput;

pub(crate) fn build(
    task_id: &str,
    specialty: Option<&str>,
    working_dir: &Path,
    model: &str,
    instruction: &str,
    read_only: bool,
    expects_changes: bool,
) -> String {
    let directory = working_dir.to_string_lossy();
    let prompt = system_prompt(SystemPromptInput {
        specialty: specialty.unwrap_or("Generalist"),
        subtask_id: task_id,
        working_dir: &directory,
        model,
        instruction,
        context: "",
        line_limit: None,
        read_only,
        expects_changes,
    });
    format!("{prompt}\n\nRecursive delegation is unavailable in this direct swarm worker.")
}

pub(super) fn user(task: &TaskInput) -> String {
    format!(
        "Task: {}\nSpecialty: {}\n\nInstruction: {}",
        task.name,
        task.specialty.as_deref().unwrap_or("Generalist execution"),
        task.instruction
    )
}
