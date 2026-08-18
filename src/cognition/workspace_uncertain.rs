//! Uncertainty ranking for the global workspace summary.

use std::cmp::Ordering;
use std::collections::HashMap;

use super::beliefs::{Belief, BeliefStatus};
use super::text_util;

/// How many uncertainties the workspace retains.
const TOP_UNCERTAINTIES: usize = 5;

/// Summarize stale or contested beliefs, contested first.
///
/// Order is deterministic: contested before uncontested, then lowest
/// confidence, then oldest update.
pub(super) fn top_uncertainties(store: &HashMap<String, Belief>) -> Vec<String> {
    let mut uncertain: Vec<&Belief> = store
        .values()
        .filter(|b| b.status == BeliefStatus::Stale || !b.contested_by.is_empty())
        .collect();
    uncertain.sort_by(|a, b| {
        (!b.contested_by.is_empty())
            .cmp(&!a.contested_by.is_empty())
            .then_with(|| {
                a.confidence
                    .partial_cmp(&b.confidence)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| a.updated_at.cmp(&b.updated_at))
    });
    uncertain
        .iter()
        .take(TOP_UNCERTAINTIES)
        .map(|b| {
            format!(
                "[{}] {}",
                b.belief_key,
                text_util::trim_for_storage(&b.claim, 80)
            )
        })
        .collect()
}
