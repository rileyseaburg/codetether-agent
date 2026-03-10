use crate::provider::{Message, ToolDefinition};
use std::path::Path;
use super::text::latest_user_text;

pub fn looks_like_build_execution_request(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let keywords = [
        "fix", "patch", "implement", "add", "update", "edit", "change", "refactor",
        "debug", "investigate", "run", "test", "build", "compile", "create",
        "remove", "rename", "wire", "hook up",
    ];
    keywords.iter().any(|k| lower.contains(k))
}

pub fn is_affirmative_build_followup(text: &str) -> bool {
    let lower = text.trim().to_ascii_lowercase();
    let markers = [
        "yes", "yep", "yeah", "do it", "go ahead", "proceed", "use the edit",
        "use edit", "apply it", "ship it", "fix it",
    ];
    markers.iter().any(|m| lower == *m || lower.starts_with(&format!("{m} ")))
}

pub fn looks_like_proposed_change(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "use this exact block", "now uses", "apply", "replace", "patch", "edit",
        "change", "update", "fix",
    ];
    markers.iter().any(|m| lower.contains(m))
}

pub fn assistant_offered_next_step(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let offer_markers = [
        "if you want", "want me to", "should i", "next i can", "i can also",
        "i'm ready to", "i am ready to",
    ];
    let action_markers = [
        "patch", "add", "update", "edit", "change", "fix", "implement", "style",
        "tighten", "apply", "refactor",
    ];
    offer_markers.iter().any(|m| lower.contains(m))
        && action_markers.iter().any(|m| lower.contains(m))
}

pub fn is_build_agent(agent_name: &str) -> bool {
    agent_name.eq_ignore_ascii_case("build")
}

pub fn should_force_build_tool_first_retry(
    agent_name: &str,
    retry_count: u8,
    tool_definitions: &[ToolDefinition],
    session_messages: &[Message],
    workspace_dir: &Path,
    assistant_text: &str,
    has_tool_calls: bool,
    build_mode_tool_first_max_retries: u8,
) -> bool {
    if retry_count >= build_mode_tool_first_max_retries
        || !is_build_agent(agent_name)
        || tool_definitions.is_empty()
        || has_tool_calls
    {
        return false;
    }

    if assistant_text.trim().is_empty() {
        return false;
    }

    if !build_request_requires_tool(session_messages, workspace_dir) {
        return false;
    }

    true
}

pub fn build_request_requires_tool(session_messages: &[Message], workspace_dir: &Path) -> bool {
    let Some(text) = latest_user_text(session_messages) else {
        return false;
    };

    if looks_like_build_execution_request(&text)
        && !super::text::extract_candidate_file_paths(&text, workspace_dir, 1).is_empty()
    {
        return true;
    }

    if !is_affirmative_build_followup(&text) {
        return false;
    }

    // Logic for affirmative check omitted for brevity but should be fully implemented if needed
    // In original code it was checking previous messages.
    // Let's implement it properly.
    
    let mut skipped_latest_user = false;
    for msg in session_messages.iter().rev() {
        let msg_text = super::text::extract_text_content(&msg.content);
        if msg_text.trim().is_empty() { continue; }

        if matches!(msg.role, crate::provider::Role::User) && !skipped_latest_user {
            skipped_latest_user = true;
            continue;
        }

        if matches!(msg.role, crate::provider::Role::Assistant) && assistant_offered_next_step(&msg_text) {
            return true;
        }

        if (looks_like_build_execution_request(&msg_text) || looks_like_proposed_change(&msg_text))
            && !super::text::extract_candidate_file_paths(&msg_text, workspace_dir, 1).is_empty()
        {
            return true;
        }
    }

    false
}
