//! Seed persona and thinker-endpoint defaults.

use super::{CreatePersonaRequest, text_util};

/// The root persona created when cognition starts with no personas.
pub(super) fn default_seed_persona() -> CreatePersonaRequest {
    CreatePersonaRequest {
        persona_id: Some("root-thinker".to_string()),
        name: "root-thinker".to_string(),
        role: "orchestrator".to_string(),
        charter: "Continuously observe, reflect, test hypotheses, and compress useful insights."
            .to_string(),
        swarm_id: Some("swarm-core".to_string()),
        parent_id: None,
        policy: None,
        tags: vec!["orchestration".to_string()],
    }
}

/// Ensure a thinker base URL ends at the chat-completions path.
pub(super) fn normalize_thinker_endpoint(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        return trimmed.to_string();
    }
    if trimmed.is_empty() {
        return "http://127.0.0.1:11434/v1/chat/completions".to_string();
    }
    format!("{trimmed}/chat/completions")
}

/// Derive a short proposal title from a thought's first non-empty line.
pub(super) fn proposal_title_from_thought(thought: &str, thought_count: u64) -> String {
    let first_line = thought
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("proposal");
    let compact = first_line
        .replace(['\t', '\r', '\n'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let trimmed = text_util::trim_for_storage(&compact, 72);
    if trimmed.is_empty() {
        format!("proposal-{thought_count}")
    } else {
        trimmed
    }
}
