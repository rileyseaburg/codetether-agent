//! Native session and approval-view bindings for interactive goal preparation.
mod provider;
use crate::session::Session;
use crate::tui::app::state::App;
use codetether_okr::OkrRepository;
use codetether_relay_okr::{PendingOkrApproval, interactive::CommandHost};
use std::sync::Arc;
struct Go<'a> {
    app: &'a mut App,
    session: &'a mut Session,
}
impl CommandHost for Go<'_> {
    fn current_model(&self) -> Option<&str> {
        self.session.metadata.model.as_deref()
    }
    fn set_model(&mut self, model: String) {
        self.session.metadata.model = Some(model);
    }
    async fn save_session(&self) -> Result<(), String> {
        self.session.save().await.map_err(|error| error.to_string())
    }
    fn repository(&mut self) -> &mut Option<Arc<OkrRepository>> {
        &mut self.app.state.okr_repository
    }
    fn push_system_message(&mut self, message: String) {
        super::push_system_message(self.app, message);
    }
    fn set_pending(&mut self, pending: PendingOkrApproval) {
        self.app.state.pending_okr_approval = Some(pending);
    }
    fn set_status(&mut self, status: String) {
        self.app.state.status = status;
    }
}
pub(super) async fn execute(app: &mut App, session: &mut Session, rest: &str) {
    codetether_relay_okr::interactive::execute(
        &mut Go { app, session },
        rest,
        crate::tui::constants::AUTOCHAT_DEFAULT_AGENTS,
    )
    .await;
}
