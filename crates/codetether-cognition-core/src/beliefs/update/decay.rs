//! Belief staleness decay during the Compress phase.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::beliefs::{Belief, BeliefStatus};
use crate::{AttentionItem, AttentionSource};

/// Decay every active belief past its review deadline and queue attention.
///
/// # Arguments
/// * `store` - Beliefs indexed by record ID.
/// * `attention` - Attention queue to extend.
/// * `now` - Time used for deadline and ranking decisions.
///
/// # Returns
/// Updates the supplied collections in place.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::beliefs::update::decay_stale_beliefs;
/// let mut store = std::collections::HashMap::new();
/// let mut attention = vec![];
/// let now = chrono::Utc::now();
/// let result = decay_stale_beliefs(&mut store, &mut attention, now);
/// assert!(attention.is_empty());
/// ```
pub fn decay_stale_beliefs(
    store: &mut HashMap<String, Belief>,
    attention: &mut Vec<AttentionItem>,
    now: DateTime<Utc>,
) {
    let stale_ids: Vec<String> = store
        .values()
        .filter(|b| b.status == BeliefStatus::Active && now > b.review_after)
        .map(|b| b.id.clone())
        .collect();

    for id in stale_ids {
        if let Some(belief) = store.get_mut(&id) {
            belief.decay();
            attention.push(stale_item(belief, now));
        }
    }
}

fn stale_item(belief: &Belief, now: DateTime<Utc>) -> AttentionItem {
    AttentionItem {
        id: Uuid::new_v4().to_string(),
        topic: format!("Stale belief: {}", belief.claim),
        topic_tags: vec![belief.belief_key.clone()],
        priority: 0.4,
        source_type: AttentionSource::StaleBelief,
        source_id: belief.id.clone(),
        assigned_persona: None,
        created_at: now,
        resolved_at: None,
    }
}
