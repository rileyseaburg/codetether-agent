//! Post-repair overflow clamp + dropped-range recompute for
//! [`super::incremental::derive_incremental`] (issue #231 item 5).

use crate::provider::{Message, ToolDefinition};
use crate::session::ResidencyLevel;
use crate::session::helper::token::estimate_request_tokens;

use super::incremental_coverage::uncovered_ranges;
use super::incremental_types::MessageOrigin;

/// Trim stale messages from the oldest side without crossing the active user
/// turn, keeping `resolutions` and `origins` in lock-step. If fixed request
/// overhead already exceeds the budget, preserving the active turn wins.
///
/// Returns `(final_dropped_ranges, provenance_tags)`. The caller
/// merges `provenance_tags` into its own provenance vector.
pub(super) fn clamp_and_recompute(
    messages: &mut Vec<Message>,
    resolutions: &mut Vec<ResidencyLevel>,
    origins: &mut Vec<MessageOrigin>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
    clone_len: usize,
    pre_dropped: &[(usize, usize)],
) -> (Vec<(usize, usize)>, Vec<&'static str>) {
    let mut est = estimate_request_tokens(system_prompt, messages, tools);
    let mut tags = Vec::new();
    let mut clamped = false;
    let mut removed = 0;
    let removable =
        super::active_tail::active_tail_start(messages, 0).min(messages.len().saturating_sub(1));
    while est > budget_tokens && messages.len() > 1 && removed < removable {
        messages.remove(0);
        resolutions.remove(0);
        origins.remove(0);
        removed += 1;
        clamped = true;
        est = estimate_request_tokens(system_prompt, messages, tools);
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
