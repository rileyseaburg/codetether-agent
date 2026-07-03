//! Unified `/agents` listing — merges TUI-spawned and agent-tool-spawned
//! sub-agents into a single view (issue #295 / #297 Part A).

use crate::tui::app::state::App;

/// Build the display lines for `/agents`, combining both registries.
pub fn build_agent_list(app: &App) -> Vec<String> {
    let mut lines = Vec::new();
    collect_tui_agents(app, &mut lines);
    collect_agent_tool_agents(&mut lines);
    lines
}

fn collect_tui_agents(app: &App, out: &mut Vec<String>) {
    for (key, agent) in &app.state.spawned_agents {
        let msg_count = agent.session.history().len();
        let model = agent.session.metadata.model.as_deref().unwrap_or("default");
        let active = if app.state.active_spawned_agent.as_deref() == Some(key.as_str()) {
            " [active]"
        } else {
            ""
        };
        out.push(format!(
            "  {}{} — {} messages — model: {}",
            agent.name, active, msg_count, model
        ));
    }
}

fn collect_agent_tool_agents(out: &mut Vec<String>) {
    let snapshots = crate::tool::agent::bridge::list_agent_tool_agents();
    let existing: std::collections::HashSet<String> =
        out.iter().filter_map(|l| name_from_line(l)).collect();
    for snap in snapshots {
        if existing.contains(&snap.name) {
            continue;
        }
        let model = snap.model_id.as_deref().unwrap_or("default");
        out.push(format!(
            "  {} — {} messages — model: {} (agent-tool)",
            snap.name, snap.message_count, model
        ));
    }
}

fn name_from_line(line: &str) -> Option<String> {
    line.trim_start()
        .split_whitespace()
        .next()
        .map(str::to_string)
}
