//! Collaboration mutations that may reach a local or LAN agent runtime.

pub(super) fn check(tool: &str) -> bool {
    matches!(
        tool,
        "spawn_agent" | "followup_task" | "resume_agent" | "send_input" | "send_message"
    )
}
