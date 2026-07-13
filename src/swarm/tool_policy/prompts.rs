use super::prompt_sections::{coordination_prompt, workflow_prompt};

use super::prompt_input::SystemPromptInput;
use std::path::Path;

pub(crate) fn system_prompt(input: SystemPromptInput<'_>) -> String {
    let project = super::project_quality::load(Path::new(input.working_dir));
    let coordination = coordination_prompt(input.read_only, input.model);
    let prd_filename = format!("prd_{}.json", input.subtask_id.replace('-', "_"));
    let workflow = workflow_prompt(input.read_only, &prd_filename);
    let quality =
        super::quality_contract::render(input.read_only, input.line_limit.or(project.line_limit));
    let verification = super::VerificationContract::from_instruction(input.instruction).prompt();
    let constraints = super::constraint_ledger::render(
        input.instruction,
        input.context,
        &project.policy,
        input.read_only,
    );
    format!(
        "You are a {} specialist sub-agent (ID: {}). You have access to tools to complete your task.\n\nWORKING DIRECTORY: {}\nAll file operations should be relative to this directory.\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\nWhen done, provide a brief summary of what you accomplished.{}",
        input.specialty,
        input.subtask_id,
        input.working_dir,
        super::capability_prompt::mode(input.read_only),
        super::capability_prompt::tools(input.read_only),
        coordination,
        workflow,
        quality,
        verification,
        constraints,
        project.instructions,
    )
}

#[cfg(test)]
#[path = "verification_prompt_tests.rs"]
mod verification_tests;
