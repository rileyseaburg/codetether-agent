//! Contradiction handling when a new belief contests existing ones.

use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::beliefs::{Belief, BeliefStatus};
use crate::{AttentionItem, AttentionSource};

/// Confidence at or above which a contested belief warrants revalidation.
const REVALIDATE_THRESHOLD: f32 = 0.5;

/// Apply `new_belief`'s contradictions to the store, queueing revalidation
/// attention for still-confident targets.
///
/// # Arguments
/// * `store` - Beliefs indexed by record ID.
/// * `attention` - Attention queue to extend.
/// * `new_belief` - New belief containing contradiction targets.
///
/// # Returns
/// Updates the supplied collections in place.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::beliefs::update::apply_contradictions;
/// let mut store = std::collections::HashMap::new();
/// let mut attention = vec![];
/// let mut new_belief = codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs("p1", r#"{"claims":[{"claim":"Service healthy","belief_key":"health","confidence":0.8}]}"#).remove(0);
/// let result = apply_contradictions(&mut store, &mut attention, &mut new_belief);
/// assert!(attention.is_empty());
/// ```
pub fn apply_contradictions(
    store: &mut HashMap<String, Belief>,
    attention: &mut Vec<AttentionItem>,
    new_belief: &mut Belief,
) {
    for target_key in new_belief.contradicts.clone() {
        let Some(target) = store
            .values_mut()
            .find(|b| b.belief_key == target_key && b.status != BeliefStatus::Invalidated)
        else {
            continue;
        };
        target.contested_by.push(new_belief.id.clone());
        if !new_belief.contradicts.contains(&target.belief_key) {
            new_belief.contradicts.push(target.belief_key.clone());
        }
        target.revalidation_failure();
        if target.confidence >= REVALIDATE_THRESHOLD {
            attention.push(revalidation_item(target));
        }
    }
}

/// Build an attention item asking a persona to revalidate `target`.
fn revalidation_item(target: &Belief) -> AttentionItem {
    AttentionItem {
        id: Uuid::new_v4().to_string(),
        topic: format!("Revalidate belief: {}", target.claim),
        topic_tags: vec![target.belief_key.clone()],
        priority: 0.7,
        source_type: AttentionSource::ContestedBelief,
        source_id: target.id.clone(),
        assigned_persona: None,
        created_at: Utc::now(),
        resolved_at: None,
    }
}
