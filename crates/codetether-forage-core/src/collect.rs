//! Rank eligible unfinished work with a caller-provided scoring hook.
use crate::{ForageOpportunity, MoonshotRubric, weights::status_weight};
use chrono::Utc;
use codetether_okr::{KeyResult, Okr};
use std::cmp::Ordering;
/// Collect and rank eligible unfinished key results.
///
/// # Arguments
/// * `okrs` — Objectives with their key results.
/// * `moonshots` — Alignment goals and optional required threshold.
/// * `score` — Hook receiving objective, key result, base score, remaining work and alignment.
/// # Returns
/// Opportunities sorted by descending score, then descending remaining work.
/// # Examples
/// ```
/// let items = codetether_forage_core::collect_opportunities_with_rubric(&[], &Default::default(), &mut |_, _, base, _, _| base);
/// assert!(items.is_empty());
/// ```
pub fn collect_opportunities_with_rubric(
    okrs: &[Okr],
    moonshots: &MoonshotRubric,
    score: &mut dyn FnMut(&Okr, &KeyResult, f64, f64, f64) -> f64,
) -> Vec<ForageOpportunity> {
    let now = Utc::now();
    let mut items = Vec::new();
    for okr in okrs {
        let status_weight = status_weight(okr.status);
        if status_weight <= 0.0 {
            continue;
        }
        for kr in &okr.key_results {
            if let Some(item) = crate::build::build(okr, kr, moonshots, now, status_weight, score) {
                items.push(item);
            }
        }
    }
    items.sort_by(
        |a, b| match b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal) {
            Ordering::Equal => b
                .remaining
                .partial_cmp(&a.remaining)
                .unwrap_or(Ordering::Equal),
            other => other,
        },
    );
    items
}
