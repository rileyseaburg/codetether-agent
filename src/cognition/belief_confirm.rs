//! Belief duplicate detection and confirmation.

use std::collections::HashMap;

use super::beliefs::{Belief, BeliefStatus};

/// Find a live belief already asserting the same key.
pub(super) fn existing_id(store: &HashMap<String, Belief>, belief: &Belief) -> Option<String> {
    store
        .values()
        .find(|b| b.belief_key == belief.belief_key && b.status != BeliefStatus::Invalidated)
        .map(|b| b.id.clone())
}

/// Record this persona as a confirming source and reward the belief.
pub(super) fn confirm_existing(store: &mut HashMap<String, Belief>, id: &str, persona_id: &str) {
    if let Some(existing) = store.get_mut(id) {
        if !existing.confirmed_by.contains(&persona_id.to_string()) {
            existing.confirmed_by.push(persona_id.to_string());
        }
        existing.revalidation_success();
    }
}
