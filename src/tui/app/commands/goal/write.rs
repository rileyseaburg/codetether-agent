//! Goal-writing command routing.

#[path = "write/change.rs"]
mod change_goal;
#[path = "write/edit.rs"]
mod edit_goal;
#[path = "write/notes.rs"]
mod notes;
#[path = "write/set.rs"]
mod set_goal;

use crate::session::tasks::GoalEditAction;
use anyhow::Result;

pub(super) async fn set(session_id: &str, objective: &str) -> Result<String> {
    set_goal::run(session_id, objective).await
}

pub(super) async fn edit(session_id: &str, objective: &str) -> Result<String> {
    edit_goal::run(session_id, objective).await
}

pub(super) async fn reaffirm(session_id: &str, note: &str) -> Result<String> {
    notes::reaffirm(session_id, note).await
}

pub(super) async fn clear(session_id: &str, reason: &str) -> Result<String> {
    notes::clear(session_id, reason).await
}

pub(super) async fn change(
    session_id: &str,
    action: GoalEditAction,
    objective: Option<String>,
    budget: Option<Option<i64>>,
) -> Result<String> {
    change_goal::run(session_id, action, objective, budget).await
}
