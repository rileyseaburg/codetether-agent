use super::prompt_sections::{coordination_prompt, workflow_prompt};

use super::prompt_input::SystemPromptInput;
use std::path::Path;

pub(crate) fn system_prompt(input: SystemPromptInput<'_>) -> String {
    let project = super::project_quality::load(Path::new(input.working_dir));
    let non_mutating = input.read_only || !input.expects_changes;
    let coordination = coordination_prompt(non_mutating, input.model);
    let prd_filename = format!("prd_{}.json", input.subtask_id.replace('-', "_"));
    let workflow = workflow_prompt(non_mutating, &prd_filename);
    let quality =
        super::quality_contract::render(non_mutating, input.line_limit.or(project.line_limit));
    let verification = super::VerificationContract::from_instruction(input.instruction).prompt();
    let deliverable = super::deliverable_contract::render();
    let metrics =
        super::source_metric_contract::render(input.specialty, input.instruction, non_mutating);
    let constraints = super::constraint_ledger::render(
        input.instruction,
        input.context,
        &project.policy,
        non_mutating,
    );
    format!(
        "You are a {} specialist sub-agent (ID: {}). You have access to tools to complete your task.\n\nWORKING DIRECTORY: {}\nAll file operations should be relative to this directory.\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\nWhen done, provide a brief evidence summary.\n\n{}",
        input.specialty,
        input.subtask_id,
        input.working_dir,
        super::capability_prompt::mode(input.read_only, input.expects_changes),
        super::capability_prompt::tools(input.read_only, input.expects_changes),
        coordination,
        workflow,
        quality,
        verification,
        metrics,
        constraints,
        project.instructions,
        deliverable,
    )
}

#[cfg(test)]
#[path = "source_metric_prompt_tests.rs"]
mod metric_tests;
#[cfg(test)]
#[path = "verification_prompt_tests.rs"]
mod verification_tests;
