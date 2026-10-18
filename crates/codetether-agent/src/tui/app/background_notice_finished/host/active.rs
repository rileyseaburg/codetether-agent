use super::{App, Session};
use codetether_session_events::completion::success::ActiveSession;

pub(crate) struct Native<'a> {
    pub app: &'a mut App,
    pub session: &'a mut Session,
}
impl ActiveSession for Native<'_> {
    fn attach_bus(&mut self) {
        self.session.attach_global_bus_if_missing();
    }
    fn reset_reconnect(&mut self) {
        crate::tui::app::background::stream_reconnect_execute::on_success(self.app);
    }
    fn set_session_id(&mut self) {
        self.app.state.session_id = Some(self.session.id.clone());
    }
    async fn save(&self) {
        let _ = self.session.save().await;
    }
    fn upsert(&mut self) {
        crate::tui::app::session_sync::upsert_active_session(self.app, self.session);
    }
}
