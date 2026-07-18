//! Observable status selection for one resident child.

use super::super::thread_status;
use crate::tool::agent::{execution_state, store::AgentEntry};
use thread_status::ThreadStatus;

pub(super) fn for_entry(entry: &AgentEntry) -> ThreadStatus {
    match thread_status::get(entry.id()) {
        ThreadStatus::NotFound if execution_state::is_running(entry.id()) => ThreadStatus::Running,
        ThreadStatus::NotFound => ThreadStatus::Completed(None),
        status => status,
    }
}
