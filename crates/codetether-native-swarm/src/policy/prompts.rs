//! Application session instructions applied during shared prompt composition.
use super::SystemPromptInput;
/// Compose task instructions with native session goal-maintenance policy.
/// # Arguments
/// * input - Subtask role, workspace and capability constraints.
/// # Returns
/// The complete sub-agent system prompt.
/// # Examples
/// Call system_prompt(input) before preparing a local or remote sub-agent job.
pub fn system_prompt(input: SystemPromptInput<'_>) -> String {
    codetether_swarm_core::prompt_policy::system_prompt(
        input,
        codetether_native_goals::turn_instructions::append,
    )
}

#[cfg(test)]
include!("prompt_tests_manifest.rs");
