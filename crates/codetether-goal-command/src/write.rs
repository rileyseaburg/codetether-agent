//! Goal-writing command routing.
use crate::Host;

#[path = "write/change.rs"]
mod change_goal;
#[path = "write/edit.rs"]
mod edit_goal;
#[path = "write/notes.rs"]
mod notes;
#[path = "write/set.rs"]
mod set_goal;

use anyhow::Result;
use codetether_goal_core::GoalEditAction;

pub(super) async fn set<H: Host>(session_id: &str, objective: &str) -> Result<String> {
    set_goal::run::<H>(session_id, objective).await
}

pub(super) async fn edit<H: Host>(session_id: &str, objective: &str) -> Result<String> {
    edit_goal::run::<H>(session_id, objective).await
}

pub(super) async fn reaffirm<H: Host>(session_id: &str, note: &str) -> Result<String> {
    notes::reaffirm::<H>(session_id, note).await
}

pub(super) async fn clear<H: Host>(session_id: &str, reason: &str) -> Result<String> {
    notes::clear::<H>(session_id, reason).await
}

pub(super) async fn change<H: Host>(
    session_id: &str,
    action: GoalEditAction,
    objective: Option<String>,
    budget: Option<Option<i64>>,
) -> Result<String> {
    change_goal::run::<H>(session_id, action, objective, budget).await
}
