//! filter for session todo tools.
use super::*;
pub(super) fn matches(t: &TodoItem, p: &ReadParams) -> bool {
    if let Some(ref status) = p.status {
        let expected = match status.as_str() {
            "pending" => TodoStatus::Pending,
            "in_progress" | "inprogress" => TodoStatus::InProgress,
            "done" => TodoStatus::Done,
            "blocked" => TodoStatus::Blocked,
            _ => return true,
        };
        if t.status != expected {
            return false;
        }
    }
    if let Some(ref priority) = p.priority {
        let expected = match priority.as_str() {
            "low" => Priority::Low,
            "medium" => Priority::Medium,
            "high" => Priority::High,
            "critical" => Priority::Critical,
            _ => return true,
        };
        if t.priority != expected {
            return false;
        }
    }
    true
}
