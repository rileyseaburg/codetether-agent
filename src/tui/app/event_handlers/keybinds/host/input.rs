use super::Native;
use codetether_tui_navigation::keys::Input;
use crossterm::event::{KeyCode, KeyEvent};
impl Input for Native<'_> {
    fn tab(&mut self, key: KeyCode) {
        super::super::super::tab_keys::dispatch(self.app, key);
    }
    async fn model_picker(&mut self) {
        super::super::super::model_picker_key::open_model_picker_if_session(
            self.app,
            self.slot,
            self.registry,
        )
        .await;
    }
    async fn enter(&mut self) {
        crate::tui::app::input::handle_enter(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
        )
        .await;
    }
    async fn other(&mut self, key: KeyEvent) {
        super::super::super::mode_keys::handle_char_or_mode_key(self.app, self.slot, key).await;
    }
}
