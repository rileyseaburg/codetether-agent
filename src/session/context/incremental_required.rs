//! Protect the active user instruction without making its entire tool history uncompressible.
use crate::session::Session;

pub(super) fn seed(
    session: &Session,
    keep: &mut [bool],
    costs: &[usize],
    recent_start: usize,
    budget: &mut usize,
) {
    super::super::state_header_pins::force_keep_base(session, keep, costs, recent_start, budget);
    if let Some(anchor) = super::super::active_tail::active_user_tail_start(&session.messages, 0)
        && !keep[anchor]
    {
        keep[anchor] = true;
        *budget = budget.saturating_sub(costs[anchor]);
    }
}

#[cfg(test)]
#[path = "incremental_required_tests.rs"]
mod tests;
