//! Goal-status-specific governance rules.

use crate::session::tasks::GoalStatus;

pub(super) fn append(output: &mut String, status: GoalStatus) {
    match status {
        GoalStatus::Active => output.push_str(
            "\nKeep working until the objective is proven complete or genuinely blocked. \
             Use `update_goal` only for those terminal transitions.\n",
        ),
        GoalStatus::BudgetLimited => output.push_str(
            "\nThe token budget is exhausted. Wrap up without starting substantive new work.\n",
        ),
        GoalStatus::Paused
        | GoalStatus::Blocked
        | GoalStatus::UsageLimited
        | GoalStatus::Complete => {
            output.push_str("\nAutomatic goal continuation is stopped for this status.\n");
        }
    }
}
