//! Goal-writing command routing.

#[path = "write/edit.rs"]
mod edit_goal;
#[path = "write/notes.rs"]
mod notes;
#[path = "write/set.rs"]
mod set_goal;

use crate::session::Session;
use anyhow::Result;

pub(super) async fn set(session: &Session, objective: &str) -> Result<String> {
    set_goal::run(session, objective).await
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
