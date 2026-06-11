//! Canned acknowledgement for inbound mesh introductions.

use crate::a2a::types::AgentCard;

/// Build the static intro acknowledgement text (no LLM involved).
pub fn intro_ack_text(card: &AgentCard) -> String {
    let skills: Vec<&str> = card.skills.iter().map(|s| s.id.as_str()).collect();
    format!(
        "Acknowledged. This is {} ({}) — A2A peer registered. Skills: {}. \
         Send a task message to collaborate.",
        card.name,
        card.url,
        skills.join(", ")
    )
}
