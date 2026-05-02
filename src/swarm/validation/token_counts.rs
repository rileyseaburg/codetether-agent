use crate::swarm::SubTask;

const CHARS_PER_TOKEN: usize = 4;
pub(super) const BASE_PROMPT_TOKENS: usize = 500;

pub(super) fn instruction_tokens(subtasks: &[SubTask]) -> usize {
    subtasks
        .iter()
        .map(|subtask| subtask.instruction.len() / CHARS_PER_TOKEN)
        .sum()
}

pub(super) fn context_tokens(subtasks: &[SubTask]) -> usize {
    subtasks
        .iter()
        .flat_map(|subtask| subtask.context.dependency_results.values())
        .map(|value| value.len() / CHARS_PER_TOKEN)
        .sum()
}
