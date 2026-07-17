//! Parent-scoped listing of managed child agents.

use crate::tui::app::state::App;

#[path = "agents_cmd.rs"]
mod rows;

pub(crate) fn list(app: &mut App) {
    if app.state.active_spawned_agent.is_none() {
        crate::tui::app::navigation::handle_tab(app);
    }
    let lines = rows::build_agent_list(app);
    let swarm_count = app.state.swarm.subtasks.len();
    let body = if lines.is_empty() && swarm_count == 0 {
        "No spawned agents. Use /spawn <name> <task>.".to_string()
    } else if lines.is_empty() {
        format!("{swarm_count} swarm worker(s) available in the dashboard")
    } else {
        lines.join("\n")
    };
    let count = lines.len() + swarm_count;
    crate::tui::app::managed_agent::ui::present(
        app,
        &crate::tool::ToolResult::success(body),
        format!("{count} spawned agent(s)"),
        "Agent listing failed",
    );
}
