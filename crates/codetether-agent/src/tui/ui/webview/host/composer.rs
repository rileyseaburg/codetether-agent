//! Bind composer cursor placement, suggestions and status data.
use super::Native;
use ratatui::{Frame, layout::Rect};
impl codetether_webview::Composer for Native<'_> {
    fn input_panel(&mut self, f: &mut Frame, area: Rect) {
        super::super::chat::render_webview_input(f, self.0, area);
    }
    fn suggestions(&mut self, f: &mut Frame, area: Rect) {
        crate::tui::ui::chat_view::suggestions::render_suggestions(f, self.0, area);
    }
    fn status(&mut self, f: &mut Frame, area: Rect) {
        super::super::status::render_webview_status(f, self.0, area);
    }
}
