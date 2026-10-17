//! Belief duplicate detection and confirmation.

use std::collections::HashMap;

use crate::beliefs::{Belief, BeliefStatus};

/// Find a live belief already asserting the same key.
///
/// # Arguments
/// * `store` - Beliefs indexed by record ID.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::beliefs::update::existing_id;
/// let mut store = std::collections::HashMap::new();
/// let belief = codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs("p1", r#"{"claims":[{"claim":"Service healthy","belief_key":"health","confidence":0.8}]}"#).remove(0);
/// let result = existing_id(&store, &belief);
/// assert!(result.is_none());
/// ```
pub fn existing_id(store: &HashMap<String, Belief>, belief: &Belief) -> Option<String> {
    store
        .values()
        .find(|b| b.belief_key == belief.belief_key && b.status != BeliefStatus::Invalidated)
        .map(|b| b.id.clone())
}

/// Record this persona as a confirming source and reward the belief.
///
/// # Arguments
/// * `store` - Beliefs indexed by record ID.
///
/// # Returns
/// Updates the supplied collections in place.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::beliefs::update::confirm_existing;
/// let mut store = std::collections::HashMap::new();
/// let result = confirm_existing(&mut store, "missing", "p1");
/// assert!(store.is_empty());
/// ```
pub fn confirm_existing(store: &mut HashMap<String, Belief>, id: &str, persona_id: &str) {
    if let Some(existing) = store.get_mut(id) {
        if !existing.confirmed_by.contains(&persona_id.to_string()) {
            existing.confirmed_by.push(persona_id.to_string());
        }
        existing.revalidation_success();
    }
}
