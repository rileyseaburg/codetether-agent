use super::{Native, Session, active};
use codetether_session_events::completion::success::Host;

impl Host for Native<'_> {
    type Session = Session;
    type Active<'a>
        = active::Native<'a>
    where
        Self: 'a;
    fn session_id(session: &Session) -> &str {
        &session.id
    }
    fn current_id(&self) -> &str {
        self.slot.view().id()
    }
    fn warn_stale(&self, session: &Session) {
        tracing::warn!(stale_id = %session.id, current_id = %self.slot.view().id(), "Discarding stale session result");
    }
    async fn save(session: &Session) {
        let _ = session.save().await;
    }
    async fn refresh(&mut self) {
        crate::tui::app::session_sync::refresh_sessions(self.app, self.cwd).await;
    }
    fn processing(&self) -> bool {
        self.app.state.processing
    }
    async fn stopped(&mut self) {
        crate::tui::app::worker_bridge::handle_processing_stopped(self.app, self.worker_bridge)
            .await;
    }
    fn clear_timing(&mut self) {
        self.app.state.request_timing.clear_request_timing();
    }
    fn restore(&mut self, session: Session) {
        self.slot.restore(session);
        crate::tui::app::inbox::live::settle_from_session(self.app, self.slot);
    }
    fn active(&mut self) -> Option<Self::Active<'_>> {
        Some(active::Native {
            app: self.app,
            session: self.slot.borrow_mut()?,
        })
    }
}
