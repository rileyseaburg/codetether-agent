//! System prompt construction for spawned agents.

use crate::tui::app::state::agent_profile;

/// Return generated child-agent instructions with report-back rules.
pub fn system_prompt(name: &str, instructions: &str) -> String {
    let mission = if instructions.is_empty() {
        default_mission(name)
    } else {
        instructions.to_string()
    };
    format!("{mission}\n\n{}", report_contract())
}

fn default_mission(name: &str) -> String {
    let p = agent_profile(name);
    format!(
        "You are an AI assistant codenamed '{}' ({}) working as a sub-agent.\n\
         Personality: {}\nCollaboration style: {}\nSignature move: {}",
        p.codename, p.profile, p.personality, p.collaboration_style, p.signature_move,
    )
}

fn report_contract() -> &'static str {
    "Report-back contract: keep work autonomous, summarize progress compactly, \
     name blockers explicitly, and end with DONE/BLOCKED plus evidence. \
     The parent TUI shows you in /agents with lineage, state, model, and mission."
}
