//! Read the durable answer hold, failing closed on unreadable task logs.

use crate::session::tasks::{TaskLog, TaskState};
use anyhow::Result;

pub(crate) fn read(session: &str) -> Result<TaskState> {
    Ok(TaskState::from_log(
        &TaskLog::for_session(session)?.read_all_blocking()?,
    ))
}

pub(crate) fn held(session: &str) -> bool {
    match read(session) {
        Ok(state) => state.answer_review.is_some(),
        Err(error) => {
            tracing::warn!(%error, session, "Answer-review log unreadable; refusing continuation");
            true
        }
    }
}

pub(crate) fn ready(session: &str) -> bool {
    read(session)
        .ok()
        .and_then(|state| state.answer_review)
        .is_some_and(|review| review.ready)
}
