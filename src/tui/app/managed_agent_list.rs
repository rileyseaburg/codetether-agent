//! Parent-scoped listing of managed child agents.

use crate::tui::app::state::App;

#[path = "agents_cmd.rs"]
mod rows;

pub(crate) fn list(app: &mut App) {
    let lines = rows::build_agent_list(app);
    let body = if lines.is_empty() {
        "No spawned agents. Use /spawn <name> <task>.".to_string()
    } else {
        lines.join("\n")
    };
    let count = lines.len();
    crate::tui::app::managed_agent::ui::present(
        app,
        &crate::tool::ToolResult::success(body),
        format!("{count} spawned agent(s)"),
        "Agent listing failed",
    );
}
