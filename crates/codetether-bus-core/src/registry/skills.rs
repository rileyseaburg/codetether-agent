use super::*;
pub(super) fn build(capabilities: &[String]) -> Vec<AgentSkill> {
    let skills = capabilities
        .iter()
        .map(|capability| {
            let id = super::skill_id::sanitize_skill_id(capability);
            AgentSkill {
                id,
                name: capability.clone(),
                description: format!("Capability: {capability}"),
                tags: vec!["protocol".to_string(), "bus".to_string()],
                examples: vec![],
                input_modes: vec!["text".to_string()],
                output_modes: vec!["text".to_string()],
            }
        })
        .collect::<Vec<_>>();

    skills
}
