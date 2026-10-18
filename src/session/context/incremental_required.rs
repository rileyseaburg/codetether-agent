//! Native session binding retained for required-entry integration tests.
use crate::session::Session;
pub(super) fn seed(
    session: &Session,
    keep: &mut [bool],
    costs: &[usize],
    recent_start: usize,
    budget: &mut usize,
) {
    codetether_incremental_derivation::seed(
        &session.messages,
        &session.pages,
        keep,
        costs,
        recent_start,
        budget,
    );
}
#[cfg(test)]
#[path = "incremental_required_tests.rs"]
mod tests;
