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
    let _guard = EDITS.lock().await;
    let before = TaskState::from_log(&log.read_all().await?);
    validate::request(&before, &edit)?;
    let at = Utc::now();
    let event = GoalEdited { at, request: edit };
    let clearing = matches!(event.request.action, GoalEditAction::Clear);
    log.append(&TaskEvent::GoalEdited(event)).await?;
    let after = TaskState::from_log(&log.read_all().await?);
    let applied = if clearing {
        after.goal.is_none()
    } else {
        after
            .goal
            .as_ref()
            .is_some_and(|goal| goal.last_updated_at == at)
    };
    if !applied {
        return Err(Error::Conflict("goal changed during save"));
    }
    Ok(after)
}
