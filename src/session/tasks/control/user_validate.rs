//! Human authority checks preserve identity, task state, and explicit budgets.

use super::{GoalControlError as Error, input};
use crate::session::tasks::{GoalEdit, GoalEditAction as Action, GoalStatus, TaskState};

pub(super) fn request(state: &TaskState, edit: &GoalEdit) -> Result<(), Error> {
    input::validate(edit)?;
    let goal = state.goal.as_ref().ok_or(Error::Conflict("no goal"))?;
    if goal.id != edit.goal_id {
        return Err(Error::Conflict(
            "refresh the replacement goal before retrying",
        ));
    }
    let force = matches!(edit.action, Action::Override | Action::ForceComplete);
    if !force && goal.last_updated_at != edit.updated_at {
        return Err(Error::Conflict("refresh the goal before retrying"));
    }
    if matches!(edit.action, Action::Resume) {
        if state.answer_review.is_some() {
            return Err(Error::Conflict("use override to dismiss the answer review"));
        }
        if goal.status == GoalStatus::Complete {
            return Err(Error::Conflict("use override to reopen a completed goal"));
        }
        if goal.token_budget.is_some_and(|cap| goal.tokens_used >= cap) {
            return Err(Error::Conflict(
                "remove or raise the exhausted budget first",
            ));
        }
    }
    Ok(())
}
