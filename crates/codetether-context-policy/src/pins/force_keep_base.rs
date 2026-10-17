//! force keep base policy.
use super::*;

/// Seed the incremental `keep` set: the recent-window tail (indices
/// `>= recent_start`) plus all pinned constraints, charging each kept
/// message against the remaining budget.
///
/// # Arguments
/// * `pages` - Page classifications.
/// * `source` - Corresponding message window.
/// * `keep` - Selection flags updated in place.
/// * `per_msg` - Token costs aligned with the selection flags.
/// * `recent_start` - First entry in the retained recent window.
/// * `budget_for_messages` - Remaining budget, reduced with saturating subtraction.
///
/// # Returns
/// Nothing; recent entries are reserved before pins.
///
/// # Examples
/// See [pinned_indices] for the page/message input shape.
pub fn force_keep_base(
    pages: &[PageKind],
    source: &[Message],
    keep: &mut [bool],
    per_msg: &[usize],
    recent_start: usize,
    budget_for_messages: &mut usize,
) {
    for i in recent_start..keep.len() {
        keep[i] = true;
        *budget_for_messages = budget_for_messages.saturating_sub(per_msg[i]);
    }
    force_keep_pins(pages, source, keep, per_msg, budget_for_messages);
}
