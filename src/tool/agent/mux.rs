//! Agent-tool access to readable mux session output.

use anyhow::Result;

use crate::tool::ToolResult;

/// Return a display-safe result containing one mux agent's terminal tail.
///
/// # Errors
///
/// Returns an error when the authenticated mux operation fails.
pub(super) async fn read(name: &str) -> Result<ToolResult> {
    Ok(match crate::mux::control::read_agent_output(name).await? {
        Some(output) => ToolResult::success(output),
        None => ToolResult::error(format!("Agent or mux session {name} not found")),
    })
}

/// Submit input already waiting in a mux-backed TUI.
pub(super) async fn interact(name: &str) -> Result<ToolResult> {
    Ok(match crate::mux::control::interact_agent(name).await? {
        Some(output) => ToolResult::success(output),
        None => ToolResult::error(format!("Agent or mux session {name} not found")),
    })
}

/// Return semantic runtime status for one mux-backed agent.
pub(super) async fn status(name: &str) -> Result<Option<ToolResult>> {
    let sessions = crate::mux::control::agent_sessions().await?;
    Ok(sessions
        .into_iter()
        .find(|item| crate::mux::control::is_agent_route(item, name))
        .map(|item| ToolResult::success(item.to_string())))
}
