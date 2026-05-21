//! Prompt construction for automatic checkpoint continuation.

use super::checkpoint::RunCheckpoint;

pub fn auto_resume_prompt(checkpoint: &RunCheckpoint) -> String {
    let url = checkpoint
        .current_browser_url
        .as_deref()
        .unwrap_or("unknown");
    let completed = list_or_default(&checkpoint.completed_actions, "none recorded");
    let blockers = list_or_default(&checkpoint.blockers, "none recorded");
    format!(
        "Continue this live CodeTether run from its structured checkpoint.\n\n\
Original objective:\n{objective}\n\n\
Session/workspace:\n- session_id: {session}\n- workspace: {workspace}\n- previous max-step budget: {budget}\n- message count at checkpoint: {messages}\n- current browser URL/tab: {url}\n\n\
Completed actions:\n{completed}\n\n\
Blockers/risks:\n{blockers}\n\n\
Next intended action:\n{next_action}\n\n\
Do not ask the user to reconstruct context. Resume from the existing session history and continue toward the original objective while preserving its constraints.",
        objective = checkpoint.original_objective,
        session = checkpoint.session_id,
        workspace = checkpoint
            .workspace
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        budget = checkpoint.max_step_budget,
        messages = checkpoint.message_count,
        url = url,
        completed = completed,
        blockers = blockers,
        next_action = checkpoint.next_intended_action,
    )
}

fn list_or_default(items: &[String], default_text: &str) -> String {
    if items.is_empty() {
        return format!("- {default_text}");
    }
    items
        .iter()
        .map(|item| format!("- {item}"))
        .collect::<Vec<_>>()
        .join("\n")
}
