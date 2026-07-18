//! Injection of persisted goal governance into provider system prompts.

use crate::session::tasks::{TaskLog, TaskState, governance_block};

pub(crate) fn compose(base: &str, session_id: &str) -> String {
    let Ok(log) = TaskLog::for_session(session_id) else { return base.into() };
    let state = TaskState::from_log(&log.read_all_blocking().unwrap_or_default());
    governance_block(&state).map_or_else(|| base.into(), |block| format!("{base}\n\n{block}"))
}
