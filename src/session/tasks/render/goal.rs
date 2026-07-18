//! Rendering of the active or stopped goal.

use crate::session::tasks::{Goal, GoalStatus};
use std::fmt::Write;

pub(super) fn append(output: &mut String, goal: &Goal) {
    let _ = write!(output, "\nSTATUS: {}\nOBJECTIVE: {}\n", goal.status.as_str(), goal.objective);
    let budget = goal.token_budget.map_or_else(|| "none".into(), |value| value.to_string());
    let remaining = goal.token_budget.map_or_else(|| "unbounded".into(),
        |limit| limit.saturating_sub(goal.tokens_used).max(0).to_string());
    let _ = write!(output, "\nPROGRESS: {} tokens / {budget}; {remaining} remaining; {} seconds; {} turns\n",
        goal.tokens_used, goal.time_used_seconds, goal.turns_used);
    if !goal.success_criteria.is_empty() {
        output.push_str("\nDONE WHEN:\n");
        goal.success_criteria.iter().for_each(|item| { let _ = writeln!(output, "- {item}"); });
    }
    output.push_str("\nFORBIDDEN:\n- Entering credentials, passwords, or OAuth codes for the user.\n");
    goal.forbidden.iter().for_each(|item| { let _ = writeln!(output, "- {item}"); });
    append_status_rule(output, goal.status);
}

fn append_status_rule(output: &mut String, status: GoalStatus) {
    match status {
        GoalStatus::Active => output.push_str(
            "\nKeep working until the objective is proven complete or genuinely blocked. \
             Use `update_goal` only for those terminal transitions.\n"),
        GoalStatus::BudgetLimited => output.push_str(
            "\nThe token budget is exhausted. Wrap up without starting substantive new work.\n"),
        GoalStatus::Paused | GoalStatus::Blocked | GoalStatus::UsageLimited | GoalStatus::Complete => {
            output.push_str("\nAutomatic goal continuation is stopped for this status.\n");
        }
    }
}
