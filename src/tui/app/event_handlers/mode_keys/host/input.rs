use super::Native;
use crate::tui::app::{input, settings};
use codetether_tui_navigation::modes::{Input, Setting};
use crossterm::event::KeyModifiers;
impl Input for Native<'_> {
    async fn setting(&mut self, setting: Setting) {
        if let Some(session) = self.slot.borrow_mut() {
            match setting {
                Setting::AutoApply => {
                    crate::tui::app::commands::toggle_auto_apply_edits(self.app, session).await;
                }
                Setting::Network => {
                    settings::toggle_network_access(self.app, session).await;
                }
                Setting::Autocomplete => {
                    settings::toggle_slash_autocomplete(self.app, session).await;
                }
            }
        }
    }
    async fn backspace(&mut self) {
        input::handle_backspace(self.app).await;
    }
    fn session_char(&mut self, modifiers: KeyModifiers, character: char) {
        input::handle_sessions_char(self.app, modifiers, character);
    }
    async fn character(&mut self, modifiers: KeyModifiers, character: char) {
        input::handle_char(self.app, modifiers, character).await;
    }
}
