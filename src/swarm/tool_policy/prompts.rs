//! Application session instructions applied during shared prompt composition.
use super::prompt_input::SystemPromptInput;
pub(crate) fn system_prompt(input: SystemPromptInput<'_>) -> String {
    codetether_swarm_core::prompt_policy::system_prompt(
        input,
        crate::session::tasks::turn_instructions::append,
    )
}

#[cfg(test)]
include!("prompt_tests_manifest.rs");
