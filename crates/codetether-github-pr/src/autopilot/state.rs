//! PR lifecycle states.
use super::*;

/// Lifecycle states for an auto-pilot PR.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
/// PR PrLifecycle data.
///
/// # Examples
/// ```
/// use codetether_github_pr::autopilot::PrLifecycle;
/// match PrLifecycle::Draft {
///     PrLifecycle::Draft => assert!(true),
///     _ => unreachable!(),
/// }
/// ```
pub enum PrLifecycle {
    Draft,
    CiRunning,
    CiFailed,
    Fixing,
    CiGreen,
    AwaitingReview,
    ReviewCommentsReceived,
    AddressingComments,
    ReadyToMerge,
    Merged,
    Failedterminal,
}
