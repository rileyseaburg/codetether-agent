//! Visible mission construction for spawned agent identities.
//! Explicit instructions are preserved; empty instructions use profile metadata.
use crate::agent_profile;

/// Return the user-facing mission stored for a managed child.
///
/// # Arguments
/// * `name` — Name used to select a default persona.
/// * `instructions` — Explicit mission text, preserved when nonempty.
///
/// # Returns
/// Explicit instructions or the selected profile's default mission.
///
/// # Examples
/// ```
/// assert_eq!(codetether_agent_identity::system_prompt("reviewer", "Check tests"), "Check tests");
/// ```
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
