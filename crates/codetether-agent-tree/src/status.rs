//! Observable status selection for one resident child.

use crate::Entry;
use codetether_collaboration_state::{execution as execution_state, thread_status};
use thread_status::ThreadStatus;

pub(super) fn for_entry<E: Entry>(entry: &E) -> ThreadStatus {
    match thread_status::get(entry.id()) {
        ThreadStatus::NotFound if execution_state::is_running(entry.id()) => ThreadStatus::Running,
        ThreadStatus::NotFound => ThreadStatus::Completed(None),
        status => status,
    }
}
