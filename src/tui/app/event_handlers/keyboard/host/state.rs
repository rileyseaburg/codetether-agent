use super::Native;
use codetether_tui_navigation::modified::State;
impl State for Native<'_> {
    fn in_chat(&self) -> bool {
        self.app.state.view_mode == crate::tui::models::ViewMode::Chat
    }
    fn watchdog_active(&self) -> bool {
        self.app.state.watchdog_notification.is_some()
    }
    fn webview(&self) -> bool {
        self.app.state.chat_layout_mode
            == crate::tui::ui::webview::layout_mode::ChatLayoutMode::Webview
    }
    fn status(&mut self) -> &mut String {
        &mut self.app.state.status
    }
}
