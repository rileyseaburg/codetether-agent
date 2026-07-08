//! Convert a proven [`ScopeItem`] into a project-palace [`Belief`].

use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::cognition::beliefs::{Belief, BeliefStatus};

use super::scope_item::ScopeItem;

/// Initial confidence for a belief backed by proven session evidence.
/// Chosen to clear the palace injection threshold (>= 0.7).
const CONFIDENCE_PROVEN: f32 = 0.75;

/// Review window for evidence-backed project beliefs.
const REVIEW_AFTER_DAYS: i64 = 30;

/// Normalize a deliverable into a canonical, deduplicable belief key.
pub(super) fn belief_key(deliverable: &str) -> String {
    deliverable
        .to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Build a new palace belief from a proven ledger item.
pub(super) fn from_item(item: &ScopeItem, session_id: &str) -> Belief {
    let now = Utc::now();
    Belief {
        id: Uuid::new_v4().to_string(),
        belief_key: belief_key(&item.deliverable),
        claim: item.deliverable.clone(),
        confidence: CONFIDENCE_PROVEN,
        evidence_refs: item.evidence.clone(),
        asserted_by: format!("session:{session_id}"),
        confirmed_by: Vec::new(),
        contested_by: Vec::new(),
        contradicts: Vec::new(),
        created_at: now,
        updated_at: now,
        review_after: now + Duration::days(REVIEW_AFTER_DAYS),
        status: BeliefStatus::Active,
    }
}
