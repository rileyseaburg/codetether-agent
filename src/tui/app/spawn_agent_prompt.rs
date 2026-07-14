//! System prompt construction for spawned agents.

use crate::tui::app::state::agent_profile;

/// Return the user-facing mission stored for a managed child.
pub fn system_prompt(name: &str, instructions: &str) -> String {
    if instructions.is_empty() {
        default_mission(name)
    } else {
        instructions.to_string()
    }
}

fn default_mission(name: &str) -> String {
    let p = agent_profile(name);
    format!(
        "You are an AI assistant codenamed '{}' ({}) working as a sub-agent.\n\
         Personality: {}\nCollaboration style: {}\nSignature move: {}",
        p.codename, p.profile, p.personality, p.collaboration_style, p.signature_move,
    )
}

#[cfg(test)]
#[path = "spawn_agent_prompt_tests.rs"]
mod tests;
