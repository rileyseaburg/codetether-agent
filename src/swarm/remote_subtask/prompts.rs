use super::*;
pub(super) fn build(payload: &RemoteSubtaskPayload, working_dir: &PathBuf) -> (String, String) {
    let specialty = if payload.specialty.is_empty() {
        "generalist".to_string()
    } else {
        payload.specialty.clone()
    };
    let working_dir_display = working_dir.display().to_string();
    let system_prompt = tool_policy::system_prompt(tool_policy::SystemPromptInput {
        specialty: &specialty,
        subtask_id: &payload.subtask_id,
        working_dir: &working_dir_display,
        model: &payload.model,
        instruction: &payload.instruction,
        context: &payload.context,
        line_limit: None,
        read_only: payload.read_only,
        expects_changes: !payload.read_only && !payload.verification,
    });
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
