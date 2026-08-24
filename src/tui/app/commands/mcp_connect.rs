//! Approval-aware `/mcp connect` command handling.

use crate::tui::app::state::App;

pub(super) async fn handle(app: &mut App, input: &str) -> bool {
    let Some(value) = input.strip_prefix("connect ") else {
        return false;
    };
    let mut parts = value.trim().splitn(2, char::is_whitespace);
    let Some(name) = parts.next().filter(|part| !part.is_empty()) else {
        app.state.status = usage();
        return true;
    };
    let Some(command) = parts.next().map(str::trim).filter(|part| !part.is_empty()) else {
        app.state.status = usage();
        return true;
    };
    let (command, approval_id) = command
        .rsplit_once(" --approval-id=")
        .map_or((command, None), |(command, id)| (command, Some(id)));
    report(app, name, command, approval_id).await;
    true
}

async fn report(app: &mut App, name: &str, command: &str, approval_id: Option<&str>) {
    let network_allowed = app.state.allow_network;
    let session_id = app.state.session_id.as_deref().unwrap_or("tui-unpinned");
    match app
        .state
        .mcp_registry
        .connect(name, command, approval_id, network_allowed, session_id)
        .await
    {
        Ok(tool_count) => {
            app.state.status = format!("Connected MCP server '{name}' ({tool_count} tools)");
            super::push_system_message(
                app,
                format!("Connected MCP server `{name}` with {tool_count} tools."),
            );
        }
        Err(error) => {
            app.state.status = format!("MCP connect failed: {error}");
            super::push_system_message(app, format!("MCP connect failed for `{name}`: {error}"));
        }
    }
}

fn usage() -> String {
    "Usage: /mcp connect <name> <command...> [--approval-id=ID]".to_string()
}
