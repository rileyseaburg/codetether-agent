//! Single-row formatting for agents spawned through the agent tool or
//! discovered as LAN peers.

use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

use crate::tool::agent::bridge::AgentSnapshot;

pub fn line(agent: &AgentSnapshot, selected: bool) -> Line<'static> {
    let state = match (agent.is_processing, agent.failed) {
        (true, _) => "working",
        (false, true) => "failed",
        (false, false) => "idle",
    };
    Line::from(vec![
        Span::raw(if selected { "› " } else { "  " }),
        Span::raw("  ".repeat(agent.depth as usize)),
        format!("{} ", agent.name).cyan().bold(),
        format!("{} ", agent.origin.lineage()).dim(),
        format!("[{}] ", agent.origin.kind()).magenta(),
        format!("[{state}] ").yellow(),
        summary(agent).dim(),
    ])
}

fn summary(agent: &AgentSnapshot) -> String {
    let mission = if agent.instructions.is_empty() {
        "spawned through agent tool"
    } else {
        &agent.instructions
    };
    format!("{} msg(s) · {mission}", agent.message_count)
}

#[cfg(test)]
#[path = "subagent_tool_row_tests.rs"]
mod tests;
