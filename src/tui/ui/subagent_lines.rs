//! Text rows for the sub-agent dashboard.

use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

use crate::tui::app::state::{AppState, SpawnedAgent};

/// Build dashboard rows for all spawned children.
pub fn lines(state: &AppState) -> Vec<Line<'static>> {
    let mut rows = header(state);
    let mut agents = state.spawned_agents.values().collect::<Vec<_>>();
    agents.sort_by_key(|agent| (&agent.parent, agent.depth, &agent.name));
    if agents.is_empty() {
        rows.push(Line::from(
            "No subagents yet. Use /spawn <name> [--parent <p>] [mission].".dim(),
        ));
    }
    rows.extend(agents.into_iter().map(agent_line));
    rows
}

fn header(state: &AppState) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            "Parent deploys with ".dim(),
            "/spawn".cyan(),
            " or ".dim(),
            "/detach".cyan(),
        ]),
        Line::from(
            format!(
                "active tasks: {} · local children: {}",
                state.active_tasks.count(),
                state.spawned_agents.len()
            )
            .green(),
        ),
        Line::from(""),
    ]
}

fn agent_line(agent: &SpawnedAgent) -> Line<'static> {
    let parent = agent.parent.as_deref().unwrap_or("main");
    let state = if agent.is_processing {
        "working"
    } else {
        "idle"
    };
    let model = agent.model_id.as_deref().unwrap_or("default model");
    let mission = if agent.instructions.is_empty() {
        "detached thread"
    } else {
        &agent.instructions
    };
    Line::from(vec![
        Span::raw("  ".repeat(agent.depth as usize)),
        format!("{} ", agent.name).cyan().bold(),
        format!("← {parent} ").dim(),
        format!("[{state}] ").yellow(),
        format!("{model} · {mission}").dim(),
    ])
}
