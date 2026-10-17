//! Restore call/result pairing after lossy context transforms.
//!
//! Missing results are injected before orphan results are removed. The stats
//! count both repairs and retain zero byte savings.
use super::ExperimentalStats;
use codetether_provider_core::{ContentPart, Message, Role};
use std::collections::HashSet;
mod drop_results;
mod fulfilled;
mod inject;
use fulfilled::collect_fulfilled_ids;
/// Repair tool-call/result consistency and return repair counts.
///
/// # Arguments
/// * `messages` - History to repair in place.
///
/// # Returns
/// Statistics counting injected and removed messages in `snippet_hits`.
///
/// # Examples
/// ```
/// use codetether_context_policy::transforms::pairing::repair_orphans;
/// assert_eq!(repair_orphans(&mut Vec::new()).snippet_hits, 0);
/// ```
pub fn repair_orphans(messages: &mut Vec<Message>) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    inject::inject(messages, &mut stats);
    drop_results::drop_results(messages, &mut stats);
    stats
}
#[cfg(test)]
mod tests;
