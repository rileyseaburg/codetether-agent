//! Terminal status selection after a child turn finishes.

use super::{ThreadStatus, get, set};

pub(crate) fn turn(agent_id: &str, response: &str, error: Option<&str>) {
    if matches!(
        get(agent_id),
        ThreadStatus::Interrupted | ThreadStatus::Shutdown | ThreadStatus::NotFound
    ) {
        return;
    }
    let next = match error {
        Some(error) => ThreadStatus::Errored(error.into()),
        None if response.trim().is_empty() => ThreadStatus::Completed(None),
        None => ThreadStatus::Completed(Some(response.into())),
    };
    set(agent_id, next);
}
