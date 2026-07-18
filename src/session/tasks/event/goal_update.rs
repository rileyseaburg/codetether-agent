//! Durable accounting and lifecycle updates for an existing goal.

use super::GoalStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Incremental runtime update folded into the current goal.
///
/// Missing optional fields mean "keep the existing value". Numeric deltas are
/// saturating additions, which makes replay deterministic after interruption.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoalRuntimeUpdate {
    /// Time at which this update was recorded.
    pub at: DateTime<Utc>,
    /// Goal receiving the update; stale updates are ignored.
    pub goal_id: String,
    /// Optional lifecycle transition.
    #[serde(default)]
    pub status: Option<GoalStatus>,
    /// Initial explicit token budget, when supplied by the user.
    #[serde(default)]
    pub token_budget: Option<i64>,
    /// Newly consumed uncached input and output tokens.
    #[serde(default)]
    pub token_delta: i64,
    /// Newly elapsed goal runtime in seconds.
    #[serde(default)]
    pub elapsed_seconds: i64,
    /// Number of automatic continuation turns started by this update.
    #[serde(default)]
    pub continuation_delta: u32,
}
