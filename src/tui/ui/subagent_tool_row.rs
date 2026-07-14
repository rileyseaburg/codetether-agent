//! Rows for agents spawned through the `agent` tool.

use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

use crate::tool::agent::bridge::AgentSnapshot;

/// Render one agent-tool child row.
pub fn line(agent: &AgentSnapshot, selected: bool) -> Line<'static> {
    let parent = agent.parent.as_deref().unwrap_or("main");
    Line::from(vec![
        Span::raw(if selected { "› " } else { "  " }),
        Span::raw("  ".repeat(agent.depth as usize)),
        format!("{} ", agent.name).cyan().bold(),
        format!("← {parent} ").dim(),
        "[tool-agent] ".magenta(),
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
