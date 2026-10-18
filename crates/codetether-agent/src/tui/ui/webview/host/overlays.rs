//! Bind approval and help overlays after the Webview panels.
use super::Native;
use ratatui::{Frame, layout::Rect};
impl codetether_webview::Overlays for Native<'_> {
    fn approval(&mut self, f: &mut Frame, area: Rect) {
        crate::tui::ui::chat_view::approval_overlay::render(f, self.0, area);
    }
    fn help(&mut self, f: &mut Frame) {
        crate::tui::help::render_help_overlay_if_needed(f, &mut self.0.state);
    }
}
