//! Apply shared child-tool presentation to the TUI.
use crate::{tool::ToolResult, tui::app::state::App};
pub(crate) fn present(app: &mut App, result: &ToolResult, success: String, failure: &str) {
    let (message, status) =
        codetether_agent_command::presentation::prepare(result, success, failure);
    app.state.messages.push(message);
    app.state.status = status;
    app.state.scroll_to_bottom();
}
