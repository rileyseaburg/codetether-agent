//! Durable parent ownership of a child's delegated goal.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Assignment of an existing goal to a child while its parent retains control.
///
/// Append [`crate::TaskEvent::GoalAssigned`] after declaring the child's goal.
/// Replay ignores assignments whose identity does not match the current goal.
/// Assignment does not change the objective, revision, status, or accounting.
///
/// # Examples
/// ```
/// use chrono::Utc;
/// use codetether_goal_core::{GoalAssignment, TaskEvent};
/// let event = TaskEvent::GoalAssigned(GoalAssignment {
///     at: Utc::now(), goal_id: "child-goal".into(),
///     parent_session_id: "parent-session".into(),
/// });
/// let json = serde_json::to_value(event).unwrap();
/// assert_eq!(json["kind"], "goal_assigned");
/// assert_eq!(json["parent_session_id"], "parent-session");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalAssignment {
    /// Time the parent assigned the goal, preserved across journal reloads.
    pub at: DateTime<Utc>,
    /// Current child goal identity; stale assignments have no effect.
    pub goal_id: String,
    /// Parent session that owns edits; the child tracks tasks and progress.
    pub parent_session_id: String,
}
}
}
}
}
}
}
}
}