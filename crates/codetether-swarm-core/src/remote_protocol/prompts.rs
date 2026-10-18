//! System and user prompts for remotely executed swarm tasks.
use super::RemoteSubtaskPayload;
use crate::prompt_policy::{SystemPromptInput, system_prompt};
use std::path::Path;

/// Build remote task prompts using shared swarm instruction policy.
/// # Arguments
/// * `payload` — Task identity, instructions, context, and capability flags.
/// * `working_dir` — Effective remote workspace.
/// * `append_session_instructions` — Application session instructions appended
///   at the existing point in system prompt construction.
/// # Returns
/// System and user prompt text with the original dependency-context formatting.
/// # Examples
/// ```no_run
/// # fn example(payload: &codetether_swarm_core::remote_protocol::RemoteSubtaskPayload, root: &std::path::Path) {
/// let (system, _) = codetether_swarm_core::remote_protocol::prompts::build(payload, root, |text| text);
/// assert!(!system.is_empty());
/// # }
/// ```
pub fn build(
    payload: &RemoteSubtaskPayload,
    working_dir: &Path,
    append_session_instructions: impl FnOnce(String) -> String,
) -> (String, String) {
    let specialty = if payload.specialty.is_empty() {
        "generalist".to_string()
    } else {
        payload.specialty.clone()
    };
    let working_dir_display = working_dir.display().to_string();
    let system_prompt = system_prompt(
        SystemPromptInput {
            specialty: &specialty,
            subtask_id: &payload.subtask_id,
            working_dir: &working_dir_display,
            model: &payload.model,
            instruction: &payload.instruction,
            context: &payload.context,
            line_limit: None,
            read_only: payload.read_only,
            expects_changes: !payload.read_only && !payload.verification,
        },
        append_session_instructions,
    );
    let user_prompt = if payload.context.trim().is_empty() {
        payload.instruction.clone()
    } else {
        format!(
            "{}\n\nContext from dependencies:\n{}",
            payload.instruction, payload.context
        )
    };

    (system_prompt, user_prompt)
}
