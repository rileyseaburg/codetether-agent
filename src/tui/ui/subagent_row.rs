//! Single-row formatting for the sub-agent dashboard.

use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

use crate::tui::app::state::SpawnedAgent;

/// Render one child agent row.
pub fn line(agent: &SpawnedAgent, selected: bool) -> Line<'static> {
    let parent = agent.parent.as_deref().unwrap_or("main");
    let state = if agent.is_processing {
        "working"
    } else {
        "idle"
    };
    Line::from(vec![
        Span::raw(if selected { "› " } else { "  " }),
        Span::raw("  ".repeat(agent.depth as usize)),
        format!("{} ", agent.name).cyan().bold(),
        format!("← {parent} ").dim(),
        format!("[{state}] ").yellow(),
        summary(agent).dim(),
    ])
}

fn summary(agent: &SpawnedAgent) -> String {
    let model = agent.model_id.as_deref().unwrap_or("default model");
    let mission = if agent.instructions.is_empty() {
        "detached thread"
    } else {
        &agent.instructions
    };
    format!("{model} · {mission}")
}
