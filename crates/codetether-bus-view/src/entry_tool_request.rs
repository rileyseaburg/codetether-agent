//! Entry builder for tool request messages.

use ratatui::style::Color;
use serde_json::Value;

use super::{entry_parts::EntryParts, truncate::truncate};

pub(super) fn request(
    request_id: &str,
    agent_id: &str,
    tool_name: &str,
    arguments: &Value,
    step: usize,
) -> EntryParts {
    let args = serde_json::to_string(arguments).unwrap_or_default();
    EntryParts::new(
        "TOOL→",
        format!("{agent_id} call {tool_name}"),
        format!(
            "Request: {request_id}\nAgent: {agent_id}\nStep: {step}\nTool: {tool_name}\nArgs: {}",
            truncate(&args, 200)
        ),
        Color::Yellow,
    )
}
