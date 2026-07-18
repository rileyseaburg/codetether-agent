//! Named lifecycle transitions shared by child execution paths.

use super::{ThreadStatus, set};
use std::fmt::Display;

pub(crate) fn running(agent_id: &str) {
    set(agent_id, ThreadStatus::Running);
}

pub(crate) fn interrupted(agent_id: &str) {
    set(agent_id, ThreadStatus::Interrupted);
}

pub(crate) fn shutdown(agent_id: &str) {
    set(agent_id, ThreadStatus::Shutdown);
}

pub(crate) fn restored(agent_id: &str) {
    set(agent_id, ThreadStatus::Completed(None));
}

pub(crate) fn track_error<T, E: Display>(agent_id: &str, result: Result<T, E>) -> Result<T, E> {
    if let Err(error) = &result {
        set(agent_id, ThreadStatus::Errored(error.to_string()));
    }
    result
}
