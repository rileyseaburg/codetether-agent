//! Agent-card skills projection.
use super::*;
pub(super) fn skills(card: &local::AgentCard) -> Vec<proto::AgentSkill> {
    card.skills
        .iter()
        .map(|s| proto::AgentSkill {
            id: s.id.clone(),
            name: s.name.clone(),
            description: s.description.clone(),
            tags: s.tags.clone(),
            examples: s.examples.clone(),
            input_modes: s.input_modes.clone(),
            output_modes: s.output_modes.clone(),
            security: vec![],
        })
        .collect()
}
