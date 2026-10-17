use super::prompt_sections::{coordination_prompt, workflow_prompt};

use super::prompt_input::SystemPromptInput;
use std::path::Path;

/// Compose a delegated prompt with project policy and sourced constraints.
///
/// # Arguments
/// * `input` - Task identity, capability flags, instructions and workspace.
/// * `append_session_instructions` - Host callback applied after the main prompt
///   and before the final evidence/status instructions.
///
/// # Returns
/// The complete prompt, using nearest project instructions when available.
///
/// # Examples
/// See [SystemPromptInput] for a complete construction example.
pub fn system_prompt(
    input: SystemPromptInput<'_>,
    append_session_instructions: impl FnOnce(String) -> String,
) -> String {
    let project = super::project_quality::load(Path::new(input.working_dir));
    let non_mutating = input.read_only || !input.expects_changes;
    let coordination = coordination_prompt(non_mutating, input.model);
    let prd_filename = format!("prd_{}.json", input.subtask_id.replace('-', "_"));
    let workflow = workflow_prompt(non_mutating, &prd_filename);
    let quality =
        super::quality_contract::render(non_mutating, input.line_limit.or(project.line_limit));
    let verification = super::VerificationContract::from_instruction(input.instruction).prompt();
    let deliverable = crate::deliverable::status_contract();
    let metrics =
        super::source_metric_contract::render(input.specialty, input.instruction, non_mutating);
    let constraints = super::constraint_ledger::render(
        input.instruction,
        input.context,
        &project.policy,
        super::constraint_ledger::Mode::from_flags(input.read_only, input.expects_changes),
    );
    let prompt = format!(
        "You are a {} specialist sub-agent (ID: {}). You have access to tools to complete your task.\n\nWORKING DIRECTORY: {}\nAll file operations should be relative to this directory.{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
        input.specialty,
        input.subtask_id,
        input.working_dir,
        codetether_agent_core::build_guidance::WORKTREE_GUIDANCE,
        super::capability_prompt::mode(input.read_only, input.expects_changes),
        super::capability_prompt::tools(input.read_only, input.expects_changes),
        coordination,
        workflow,
        quality,
        verification,
        metrics,
        constraints,
        project.instructions,
    );
    let prompt = append_session_instructions(prompt);
    format!("{prompt}\n\nWhen done, provide a brief evidence summary.\n\n{deliverable}")
}
