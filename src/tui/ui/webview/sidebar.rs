//! Compose workspace and recent-session panels with shared sidebar geometry.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};

pub fn render_webview_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let chunks = codetether_chat_layout::webview::sidebar_chunks(area);
    super::workspace_panel::render(f, app, chunks[0]);
    super::sessions_panel::render(f, app, chunks[1]);
}
