//! Native revision and lifecycle preconditions for user goal edits.

use super::{error::GoalControlError as Error, input};
use crate::session::tasks::{GoalEdit, GoalEditAction, GoalStatus, TaskState};

/// Check the displayed revision and reject completion or user-hold bypasses.
pub(super) fn request(state: &TaskState, edit: &GoalEdit) -> Result<(), Error> {
    if matches!(
        edit.action,
        GoalEditAction::Override | GoalEditAction::ForceComplete
    ) {
        return Err(Error::Invalid("override requires a user/admin control"));
    }
    input::validate(edit)?;
    let goal = state.goal.as_ref().ok_or(Error::Conflict("no goal"))?;
    if goal.id != edit.goal_id || goal.last_updated_at != edit.updated_at {
        return Err(Error::Conflict("refresh the goal before retrying"));
    }
    if matches!(edit.action, GoalEditAction::Clear) {
        return Ok(());
    }
    if goal.status == GoalStatus::Complete {
        return Err(Error::Conflict("completed goals cannot be resumed"));
    }
    if state.answer_review.is_some() {
        return Err(Error::Conflict("answer the pending native review first"));
    }
    if matches!(edit.action, GoalEditAction::Resume)
        && goal.token_budget.is_some_and(|cap| goal.tokens_used >= cap)
    {
        return Err(Error::Conflict(
            "remove or raise the exhausted budget first",
        ));
    }
    Ok(())
}
