//! Native UI updates after selecting or importing sessions.
use crate::{session::Session, tui::app::state::App};
use std::path::Path;
impl codetether_session_resume::interactive::View<Session> for App {
    fn set_preferences(&mut self, auto_apply: bool, worktree: bool) {
        self.state.preferences.auto_apply_edits = auto_apply;
        self.state.preferences.use_worktree = worktree;
    }
    fn set_session_id(&mut self, id: String) {
        self.state.session_id = Some(id);
    }
    fn sync_messages(&mut self, session: &Session, source_id: &str, source_has_older: bool) {
        crate::tui::app::message_text::sync_messages_from_source(
            self,
            session,
            source_id,
            source_has_older,
        );
    }
    async fn refresh(&mut self, cwd: &Path) {
        crate::tui::app::session_sync::refresh_sessions(self, cwd).await;
    }
    fn clear_filter(&mut self) {
        self.state.clear_session_filter();
    }
    fn return_to_chat(&mut self) {
        crate::tui::app::session_sync::return_to_chat(self);
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
}
