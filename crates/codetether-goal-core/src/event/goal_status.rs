//! Persisted lifecycle states for a thread goal.

use serde::{Deserialize, Serialize};

/// Lifecycle state controlling whether a goal continues automatically.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::GoalStatus;
/// assert!(GoalStatus::Active.is_active());
/// assert!(GoalStatus::Complete.is_terminal());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalStatus {
    /// The runtime should continue pursuing the objective.
    #[default]
    Active,
    /// The user temporarily stopped automatic continuation.
    Paused,
    /// The objective passed its completion audit.
    Complete,
    /// Repeated external constraints prevent meaningful progress.
    Blocked,
    /// Provider or account usage limits stopped execution.
    UsageLimited,
    /// The explicitly requested token budget was exhausted.
    BudgetLimited,
}

impl GoalStatus {
    /// Returns whether automatic continuation is allowed.
    pub fn is_active(self) -> bool {
        self == Self::Active
    }

    /// Returns whether the model may replace this goal.
    pub fn is_terminal(self) -> bool {
        self == Self::Complete
    }

    /// Stable serialized name used in tool responses.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Complete => "complete",
            Self::Blocked => "blocked",
            Self::UsageLimited => "usage_limited",
            Self::BudgetLimited => "budget_limited",
        }
    }
}
