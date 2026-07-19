//! Rows for agents spawned through the `agent` tool.

use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

use crate::tool::agent::bridge::AgentSnapshot;

/// Render one agent-tool child row.
pub fn line(agent: &AgentSnapshot, selected: bool) -> Line<'static> {
    let parent = agent.parent.as_deref().unwrap_or("main");
    let state = match (agent.is_processing, agent.failed) {
        (true, _) => "working",
        (false, true) => "failed",
        (false, false) => "idle",
    };
    let kind = if agent.is_remote {
        "[LAN peer] "
    } else {
        "[tool-agent] "
    };
    Line::from(vec![
        Span::raw(if selected { "› " } else { "  " }),
        Span::raw("  ".repeat(agent.depth as usize)),
        format!("{} ", agent.name).cyan().bold(),
        format!("← {parent} ").dim(),
        kind.magenta(),
        format!("[{state}] ").yellow(),
        summary(agent).dim(),
    ])
}

fn summary(agent: &AgentSnapshot) -> String {
    let model = agent.model_id.as_deref().unwrap_or("default model");
    let mission = if agent.instructions.is_empty() {
        "spawned through agent tool"
    } else {
        &agent.instructions
    };
    format!("{model} · {} msg(s) · {mission}", agent.message_count)
}

#[cfg(test)]
#[path = "subagent_tool_row_tests.rs"]
mod tests;
