//! Pinned-constraint index + force-keep helper shared across policies.

use crate::session::Session;
use crate::session::pages::PageKind;

/// Indices of pinned [`PageKind::Constraint`] turns in the canonical
/// transcript. Shared with the incremental scorer so pins are force-kept.
pub(super) fn pinned_indices(session: &Session) -> Vec<usize> {
    session
        .pages
        .iter()
        .zip(session.messages.iter())
        .enumerate()
        .filter(|(_, (kind, _))| matches!(kind, PageKind::Constraint))
        .map(|(idx, _)| idx)
        .collect()
}

/// Force every pinned constraint into the incremental `keep` set,
/// charging its token cost against the remaining message budget.
pub(super) fn force_keep_pins(
    session: &Session,
    keep: &mut [bool],
    per_msg: &[usize],
    budget_for_messages: &mut usize,
) {
    for i in pinned_indices(session) {
        if i < keep.len() && !keep[i] {
            keep[i] = true;
            *budget_for_messages = budget_for_messages.saturating_sub(per_msg[i]);
        }
    }
}

/// Seed the incremental `keep` set: the recent-window tail (indices
/// `>= recent_start`) plus all pinned constraints, charging each kept
/// message against the remaining budget.
pub(super) fn force_keep_base(
    session: &Session,
    keep: &mut [bool],
    per_msg: &[usize],
    recent_start: usize,
    budget_for_messages: &mut usize,
) {
    for i in recent_start..keep.len() {
        keep[i] = true;
        *budget_for_messages = budget_for_messages.saturating_sub(per_msg[i]);
    }
    force_keep_pins(session, keep, per_msg, budget_for_messages);
}
