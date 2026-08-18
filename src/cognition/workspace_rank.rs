//! Belief ranking for the global workspace summary.

use chrono::{DateTime, Utc};
use std::cmp::Ordering;
use std::collections::HashMap;

use super::beliefs::{Belief, BeliefStatus};

/// How many top beliefs the workspace retains.
const TOP_BELIEFS: usize = 10;

/// Rank active beliefs by confidence, discounted by staleness.
pub(super) fn top_belief_ids(store: &HashMap<String, Belief>, now: DateTime<Utc>) -> Vec<String> {
    let mut active: Vec<&Belief> = store
        .values()
        .filter(|b| b.status == BeliefStatus::Active)
        .collect();
    active.sort_by(|a, b| {
        recency_score(b, now)
            .partial_cmp(&recency_score(a, now))
            .unwrap_or(Ordering::Equal)
    });
    active
        .iter()
        .take(TOP_BELIEFS)
        .map(|b| b.id.clone())
        .collect()
}

/// Confidence discounted by minutes since the last update.
fn recency_score(belief: &Belief, now: DateTime<Utc>) -> f32 {
    let age_minutes = now.signed_duration_since(belief.updated_at).num_minutes() as f32;
    belief.confidence * (1.0 / (1.0 + age_minutes))
}
