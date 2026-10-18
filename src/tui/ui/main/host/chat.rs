//! Bind native chat layouts and session data.
use super::Native;
use ratatui::Frame;
impl codetether_tui_render::Chat for Native<'_> {
    fn layout(&self) -> codetether_chat_layout::webview::ChatLayoutMode {
        self.app.state.chat_layout_mode
    }
    fn webview(&mut self, frame: &mut Frame) -> bool {
        crate::tui::ui::webview::render(frame, self.app)
    }
    fn classic(&mut self, frame: &mut Frame) {
        crate::tui::ui::chat_view::render_chat_view(frame, self.app, self.session);
    }
}
