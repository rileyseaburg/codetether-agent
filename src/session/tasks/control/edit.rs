//! Serialized user edits through the native append-only task log.

use super::{error::GoalControlError as Error, validate};
use crate::session::tasks::{GoalEdit, GoalEditAction, GoalEdited, TaskEvent};
use crate::session::tasks::{TaskLog, TaskState};
use chrono::Utc;
use tokio::sync::Mutex;

static EDITS: Mutex<()> = Mutex::const_new(());

/// Append once and report only the state actually accepted during replay.
///
/// # Errors
/// Rejects stale goals, invalid input, held reviews, and storage failures.
pub(super) async fn apply(log: &TaskLog, edit: GoalEdit) -> Result<TaskState, Error> {
    apply_with(log, edit, validate::request).await
}

pub(super) async fn apply_user(log: &TaskLog, edit: GoalEdit) -> Result<TaskState, Error> {
    apply_with(log, edit, super::user_validate::request).await
}

async fn apply_with(
    log: &TaskLog,
    edit: GoalEdit,
    validate: fn(&TaskState, &GoalEdit) -> Result<(), Error>,
) -> Result<TaskState, Error> {
    let _guard = EDITS.lock().await;
    let before = TaskState::from_log(&log.read_all().await?);
    validate(&before, &edit)?;
    let at = Utc::now();
    let event = GoalEdited { at, request: edit };
    let clearing = matches!(event.request.action, GoalEditAction::Clear);
    log.append(&TaskEvent::GoalEdited(event)).await?;
    let events = log.read_all().await?;
    if !super::accepted::edit(&events, at, clearing) {
        return Err(Error::Conflict("goal changed during save"));
    }
    Ok(TaskState::from_log(&events))
}
