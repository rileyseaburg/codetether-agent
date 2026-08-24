//! First-class collaboration tools delegated through the agent backend.

pub(super) fn self_verifying(tool_name: &str) -> bool {
    matches!(
        tool_name,
        "spawn_agent"
            | "followup_task"
            | "close_agent"
            | "interrupt_agent"
            | "resume_agent"
            | "send_input"
            | "send_message"
    )
}
