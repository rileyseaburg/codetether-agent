//! Lifecycle transitions for CI and review events.
use super::*;

/// One tick of the auto-pilot state machine.
///
/// # Arguments
/// * `current` - Current lifecycle state.
/// * `ci_green` - Whether CI reports success.
/// * `has_review_comments` - Whether review comments require attention.
/// # Returns
/// The next state according to the existing lifecycle transition table.
/// # Examples
/// ```
/// use codetether_github_pr::autopilot::{PrLifecycle, advance_lifecycle};
/// assert_eq!(advance_lifecycle(&PrLifecycle::Draft, false, false), PrLifecycle::CiRunning);
/// ```
pub fn advance_lifecycle(
    current: &PrLifecycle,
    ci_green: bool,
    has_review_comments: bool,
) -> PrLifecycle {
    match current {
        PrLifecycle::Draft => PrLifecycle::CiRunning,
        PrLifecycle::CiRunning if ci_green => PrLifecycle::CiGreen,
        PrLifecycle::CiRunning => PrLifecycle::CiFailed,
        PrLifecycle::CiFailed => PrLifecycle::Fixing,
        PrLifecycle::Fixing => PrLifecycle::CiRunning,
        PrLifecycle::CiGreen if has_review_comments => PrLifecycle::ReviewCommentsReceived,
        PrLifecycle::CiGreen => PrLifecycle::AwaitingReview,
        PrLifecycle::ReviewCommentsReceived => PrLifecycle::AddressingComments,
        PrLifecycle::AddressingComments => PrLifecycle::CiRunning,
        PrLifecycle::AwaitingReview if has_review_comments => PrLifecycle::ReviewCommentsReceived,
        PrLifecycle::AwaitingReview => PrLifecycle::ReadyToMerge,
        other => other.clone(),
    }
}
