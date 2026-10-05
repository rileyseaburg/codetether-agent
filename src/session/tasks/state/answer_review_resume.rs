//! Restore the interrupted status without bypassing an exhausted token budget.

use super::Goal;
use crate::session::tasks::GoalStatus;

pub(super) fn status(goal: &Goal, previous: GoalStatus) -> GoalStatus {
    if previous.is_active()
        && goal
            .token_budget
            .is_some_and(|limit| goal.tokens_used >= limit)
    {
        GoalStatus::BudgetLimited
    } else {
        previous
    }
}
