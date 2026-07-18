//! Materialized persisted goal data.

use crate::session::tasks::GoalStatus;
use chrono::{DateTime, Utc};

/// Current persisted thread goal and its accounting totals.
///
/// # Examples
///
/// ```rust
/// use chrono::Utc;
/// use codetether_agent::session::tasks::{Goal, GoalStatus};
///
/// let now = Utc::now();
/// let goal = Goal {
///     id: "g1".into(), objective: "Ship safely".into(),
///     success_criteria: vec![], forbidden: vec![], status: GoalStatus::Active,
///     token_budget: None, tokens_used: 0, time_used_seconds: 0, turns_used: 1,
///     set_at: now, last_updated_at: now, last_reaffirmed_at: now,
/// };
/// assert!(goal.status.is_active());
/// ```
#[derive(Clone, Debug)]
pub struct Goal {
    /// Stable identifier used to reject stale runtime updates.
    pub id: String,
    /// User-authored objective preserved across turns.
    pub objective: String,
    /// Evidence requirements that prove completion.
    pub success_criteria: Vec<String>,
    /// Explicitly prohibited actions.
    pub forbidden: Vec<String>,
    /// Current lifecycle state.
    pub status: GoalStatus,
    /// Optional user-requested token ceiling.
    pub token_budget: Option<i64>,
    /// Total provider tokens charged while active.
    pub tokens_used: i64,
    /// Total elapsed provider-call time while active.
    pub time_used_seconds: i64,
    /// Initial turn plus automatic continuations.
    pub turns_used: u32,
    /// Time the objective was created.
    pub set_at: DateTime<Utc>,
    /// Time the latest runtime update was applied.
    pub last_updated_at: DateTime<Utc>,
    /// Time the latest progress reaffirmation was recorded.
    pub last_reaffirmed_at: DateTime<Utc>,
}
