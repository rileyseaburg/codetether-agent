//! Native chat reset and listing refresh bindings for new-session creation.
use crate::tui::{app::state::App, models::ViewMode};
use codetether_new_session::Preferences;
use std::path::Path;
pub(super) struct NewView<'a>(pub(super) &'a mut App);
impl codetether_new_session::View for NewView<'_> {
    fn preferences(&self) -> Preferences {
        Preferences {
            auto_apply_edits: self.0.state.preferences.auto_apply_edits,
            allow_network: self.0.state.preferences.allow_network,
            slash_autocomplete: self.0.state.preferences.slash_autocomplete,
            use_worktree: self.0.state.preferences.use_worktree,
        }
    }
    fn set_status(&mut self, status: String) {
        self.0.state.status = status;
    }
    fn set_session_id(&mut self, id: String) {
        self.0.state.session_id = Some(id);
    }
    fn clear_messages(&mut self) {
        self.0.state.messages.clear();
    }
    fn clear_streaming_text(&mut self) {
        self.0.state.streaming.streaming_text.clear();
    }
    fn clear_processing(&mut self) {
        self.0.state.processing = false;
    }
    fn clear_request_timing(&mut self) {
        self.0.state.request_timing.clear_request_timing();
    }
    fn scroll_to_bottom(&mut self) {
        self.0.state.chat_viewport.scroll_to_bottom();
    }
    fn show_chat(&mut self) {
        crate::tui::app::state::view::transition(&mut self.0.state).set_view_mode(ViewMode::Chat);
    }
    async fn refresh_sessions(&mut self, cwd: &Path) {
        crate::tui::app::session_sync::refresh_sessions(self.0, cwd).await;
    }
}
