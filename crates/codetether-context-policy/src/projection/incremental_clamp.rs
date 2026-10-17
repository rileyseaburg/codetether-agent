//! Post-repair overflow clamp + dropped-range recompute for
//! the incremental derivation coordinator (issue #231 item 5).

use crate::pages::ResidencyLevel;
use codetether_provider_core::Message;

use super::incremental_coverage::uncovered_ranges;
use super::incremental_types::MessageOrigin;

/// Trim stale messages from the oldest side without crossing the active user
/// turn, keeping `resolutions` and `origins` in lock-step. If fixed request
/// overhead already exceeds the budget, preserving the active turn wins.
///
/// Returns `(final_dropped_ranges, provenance_tags)`. The caller
/// merges `provenance_tags` into its own provenance vector.
pub fn clamp_and_recompute(
    messages: &mut Vec<Message>,
    resolutions: &mut Vec<ResidencyLevel>,
    origins: &mut Vec<MessageOrigin>,
    budget_tokens: usize,
    clone_len: usize,
    pre_dropped: &[(usize, usize)],
    estimate_request: impl Fn(&[Message]) -> usize,
) -> (Vec<(usize, usize)>, Vec<&'static str>) {
    let mut est = estimate_request(messages);
    let mut tags = Vec::new();
    let mut clamped = false;
    let mut removed = 0;
    let removable = crate::selection::active_tail::active_tail_start(messages, 0)
        .min(messages.len().saturating_sub(1));
    while est > budget_tokens && messages.len() > 1 && removed < removable {
        messages.remove(0);
        resolutions.remove(0);
        origins.remove(0);
        removed += 1;
        clamped = true;
        est = estimate_request(messages);
    }
    if clamped {
        tags.push("incremental_overflow_clamp");
    }
    let final_dropped = uncovered_ranges(origins, clone_len);
    if final_dropped != pre_dropped {
        tags.push("incremental_dropped_recomputed");
    }
    (final_dropped, tags)
}
