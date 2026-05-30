//! Small routing helpers for agent and model selection.

pub fn model_ref_to_provider_model(model: &str) -> String {
    if !model.contains('/') && model.contains(':') {
        model.replacen(':', "/", 1)
    } else {
        model.to_string()
    }
}

pub fn is_swarm_agent(agent_type: &str) -> bool {
    matches!(
        agent_type.trim().to_ascii_lowercase().as_str(),
        "swarm" | "parallel" | "multi-agent"
    )
}

pub fn is_forage_agent(agent_type: &str) -> bool {
    agent_type.trim().eq_ignore_ascii_case("forage")
}
