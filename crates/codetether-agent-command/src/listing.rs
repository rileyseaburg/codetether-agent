//! Present the combined child-agent roster.
use crate::Host;
/// List visible agents, opening the dashboard when no child is focused.
/// # Arguments
/// `app` supplies ordered roster snapshots and presentation callbacks.
/// # Returns
/// Presents the roster and total child/swarm count.
/// # Examples
/// ```no_run
/// fn command<H: codetether_agent_command::Host>(host: &mut H) {
///     codetether_agent_command::list(host);
/// }
/// ```
pub fn list<H: Host>(app: &mut H) {
    if app.active().is_none() {
        app.navigate_agents();
    }
    let lines = crate::roster::build_agent_list(app);
    let swarm_count = app.swarm_count();
    let body = if lines.is_empty() && swarm_count == 0 {
        "No spawned agents. Use /spawn <name> <task>.".to_string()
    } else if lines.is_empty() {
        format!("{swarm_count} swarm worker(s) available in the dashboard")
    } else {
        lines.join("\n")
    };
    let count = lines.len() + swarm_count;
    app.present(
        &codetether_tool_core::ToolResult::success(body),
        format!("{count} spawned agent(s)"),
        "Agent listing failed",
    );
}
