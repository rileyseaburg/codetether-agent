//! Rendering of open session tasks.

use crate::session::tasks::{SessionTaskStatus, TaskState};
use std::fmt::Write;

pub(super) fn append(output: &mut String, state: &TaskState) {
    let open = state.open_tasks();
    if open.is_empty() {
        return;
    }
    output.push_str("\nOPEN TASKS:\n");
    for task in open {
        let marker = match task.status {
            SessionTaskStatus::InProgress => "◐",
            SessionTaskStatus::Pending => "○",
            SessionTaskStatus::Done | SessionTaskStatus::Blocked | SessionTaskStatus::Cancelled => "·",
        };
        let _ = writeln!(output, "{marker} [{}] {}", task.id, task.content);
    }
}
