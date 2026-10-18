use super::Native;
use crate::tui::app::{navigation::Navigation, symbols};
use codetether_tui_navigation::keys::State;
impl State for Native<'_> {
    type Navigation<'a>
        = Navigation<'a>
    where
        Self: 'a;
    fn navigation(&mut self) -> Navigation<'_> {
        Navigation(self.app)
    }
    fn in_chat(&self) -> bool {
        self.app.state.view.mode == crate::tui::models::ViewMode::Chat
    }
    fn symbols_active(&self) -> bool {
        symbols::symbol_search_active(self.app)
    }
    fn symbol_enter(&mut self) {
        crate::tui::app::navigation::handle_symbol_enter(self.app);
    }
    fn newline(&mut self) {
        self.app.state.composer.insert_char('\n');
    }
    fn paste_newline(&self) -> bool {
        super::super::super::paste_burst::enter_is_likely_paste_newline(self.app)
    }
    fn watchdog_active(&self) -> bool {
        self.app.state.watchdog.notification.is_some()
    }
    fn dismiss_watchdog(&mut self) {
        crate::tui::app::watchdog::handle_watchdog_dismiss(&mut self.app.state);
    }
}
