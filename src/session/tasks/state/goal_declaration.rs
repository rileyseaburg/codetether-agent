//! Goal declarations respect a pending answer review's user provenance.

use super::super::{TaskState, goal};
use crate::session::tasks::{GoalSourceKind, TaskEvent};

/// Preserve the existing review hold unless the new declaration is user-owned.
pub(super) fn apply(state: &mut TaskState, event: &TaskEvent) {
    let TaskEvent::GoalSet {
        at,
        goal_id,
        objective,
        success_criteria,
        forbidden,
        source_kind,
        ..
    } = event
    else {
        return;
    };
    if state.answer_review.is_some() && *source_kind != GoalSourceKind::UserProvided {
        return;
    }
    goal::set(state, *at, goal_id, objective, success_criteria, forbidden);
    state.answer_review = None;
}
