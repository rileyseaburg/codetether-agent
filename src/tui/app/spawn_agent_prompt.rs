//! System prompt construction for spawned agents.

use crate::tui::app::state::agent_profile;

/// Return explicit instructions or a generated codename profile prompt.
pub fn system_prompt(name: &str, instructions: &str) -> String {
    if !instructions.is_empty() {
        return instructions.to_string();
    }
    let p = agent_profile(name);
    format!(
        "You are an AI assistant codenamed '{}' ({}) working as a sub-agent.\n\
         Personality: {}\nCollaboration style: {}\nSignature move: {}",
        p.codename, p.profile, p.personality, p.collaboration_style, p.signature_move,
    )
}
