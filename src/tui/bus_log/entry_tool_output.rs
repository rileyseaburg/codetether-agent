//! Entry builders for retained tool responses and full output.

use ratatui::style::Color;

use super::{entry_parts::EntryParts, truncate::truncate};

pub(super) fn response(
    request_id: &str,
    agent_id: &str,
    tool_name: &str,
    result: &str,
    success: bool,
    step: usize,
) -> EntryParts {
    let icon = if success { "✓" } else { "✗" };
    let detail = crate::tui::bus_log_entry_payload::tool_response(
        request_id, agent_id, step, tool_name, success, result,
    );
    EntryParts::new(
        "←TOOL",
        format!("{icon} {agent_id} {tool_name}"),
        detail,
        color(success),
    )
}

pub(super) fn full(
    agent_id: &str,
    tool_name: &str,
    output: &str,
    success: bool,
    step: usize,
) -> EntryParts {
    let icon = if success { "✓" } else { "✗" };
    let preview = truncate(output, 120);
    let detail =
        crate::tui::bus_log_entry_payload::tool_full(agent_id, tool_name, step, success, output);
    EntryParts::new(
        "TOOL•FULL",
        format!("{icon} {agent_id} step {step} {tool_name}: {preview}"),
        detail,
        color(success),
    )
}

fn color(success: bool) -> Color {
    if success { Color::Green } else { Color::Red }
}
