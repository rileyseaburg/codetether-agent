//! Native slot ownership and loading for shared session selection.
use super::Native;
use codetether_session_picker::{SessionPickerState, interaction::Host};
impl Host for Native<'_> {
    fn picker(&self) -> &SessionPickerState {
        &self.app.state.session_picker
    }
    async fn load_selected(&mut self, session_id: &str) {
        if let Some(session) = self.slot.borrow_mut() {
            crate::tui::app::codex_sessions::load_selected_session(
                self.app, self.cwd, session, session_id,
            )
            .await;
        }
    }
    fn current_session_id(&self) -> Option<&str> {
        self.slot.borrow().map(|session| session.id.as_str())
    }
    async fn resume(&mut self) -> bool {
        let available = self.registry.is_some();
        codetether_chat_submit::resume::automatic::run(self, available).await
    }
}
