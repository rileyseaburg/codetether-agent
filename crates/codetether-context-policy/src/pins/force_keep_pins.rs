//! force keep pins policy.
use super::*;

/// Force every pinned constraint into the incremental `keep` set,
/// charging its token cost against the remaining message budget.
///
/// # Arguments
/// * `pages` - Page classifications.
/// * `source` - Corresponding message window.
/// * `keep` - Selection flags updated in place.
/// * `per_msg` - Token costs aligned with the selection flags.
/// * `budget_for_messages` - Remaining budget, reduced with saturating subtraction.
///
/// # Returns
/// Nothing; newly retained pins update the flags and budget.
///
/// # Examples
/// See [pinned_indices] for the page/message input shape.
pub fn force_keep_pins(
    pages: &[PageKind],
    source: &[Message],
    keep: &mut [bool],
    per_msg: &[usize],
    budget_for_messages: &mut usize,
) {
    for i in pinned_indices(pages, source) {
        if i < keep.len() && !keep[i] {
            keep[i] = true;
            *budget_for_messages = budget_for_messages.saturating_sub(per_msg[i]);
        }
    }
}
