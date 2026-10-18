//! Native rendering adapter for the shared compact inspector.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};

pub fn render_webview_inspector(f: &mut Frame, app: &App, area: Rect) {
    codetether_inspector_view::compact::render(f, &super::inspector_lines::view(app), area);
}
