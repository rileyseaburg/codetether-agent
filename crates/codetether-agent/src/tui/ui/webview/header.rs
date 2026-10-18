//! Project native conversation labels into the shared header renderer.
use crate::tui::app::state::App;
use codetether_webview_bars::header::{Header, render};
use ratatui::{Frame, layout::Rect};
pub fn render_webview_header(f: &mut Frame, app: &App, area: Rect) {
    render(
        f,
        Header {
            session: app.state.session_id.as_deref(),
            agent: app.state.agent_tree.active.as_deref(),
            branch: app.state.workspace.git_branch.as_deref(),
            message_count: app.state.messages.len(),
        },
        area,
    );
}
