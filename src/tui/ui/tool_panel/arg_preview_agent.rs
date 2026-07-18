//! Compact first-party collaboration labels for the tool panel.

use serde_json::Value;

pub(super) fn format(value: &Value) -> String {
    let action = text(value, "action");
    let name = text(value, "name");
    match action.as_str() {
        "list" => "discover first-party collaborators".to_string(),
        "message" => format!("ask @{name} — {}", preview(text(value, "message"))),
        "spawn" => format!("spawn @{name}"),
        "status" => format!("check @{name}"),
        "kill" => format!("stop @{name}"),
        _ => format!("agent {action}"),
    }
}

fn text(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn preview(value: String) -> String {
    crate::tui::app::text::truncate_preview(&value.replace('\n', " "), 72)
}
