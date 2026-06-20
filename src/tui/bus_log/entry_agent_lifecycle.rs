//! Entry builders for agent lifecycle and heartbeat messages.

use ratatui::style::Color;

use super::entry_parts::EntryParts;

pub(super) fn ready(agent_id: &str, capabilities: &[String]) -> EntryParts {
    EntryParts::new(
        "READY",
        format!("{agent_id} online ({} caps)", capabilities.len()),
        format!(
            "Agent: {agent_id}\nCapabilities: {}",
            capabilities.join(", ")
        ),
        Color::Green,
    )
}

pub(super) fn shutdown(agent_id: &str) -> EntryParts {
    EntryParts::new(
        "SHUTDOWN",
        format!("{agent_id} shutting down"),
        format!("Agent: {agent_id}"),
        Color::Red,
    )
}

pub(super) fn heartbeat(agent_id: &str, status: &str) -> EntryParts {
    let is_a2a = status.starts_with("discovered via A2A");
    let color = if is_a2a {
        Color::LightCyan
    } else {
        Color::DarkGray
    };
    let kind = if is_a2a { "A2A•PEER" } else { "BEAT" };
    EntryParts::new(
        kind,
        format!("{agent_id} [{status}]"),
        format!("Agent: {agent_id}\nStatus: {status}"),
        color,
    )
}
