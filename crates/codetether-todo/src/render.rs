//! render for session todo tools.
use super::*;
pub(super) fn format(filtered: &[&TodoItem]) -> String {
    filtered
        .iter()
        .map(|t| {
            let status_icon = match t.status {
                TodoStatus::Pending => "○",
                TodoStatus::InProgress => "◐",
                TodoStatus::Done => "●",
                TodoStatus::Blocked => "✗",
            };
            let priority_label = match t.priority {
                Priority::Low => "[low]",
                Priority::Medium => "",
                Priority::High => "[HIGH]",
                Priority::Critical => "[CRITICAL]",
            };
            format!("{} {} {} {}", status_icon, t.id, priority_label, t.content)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
