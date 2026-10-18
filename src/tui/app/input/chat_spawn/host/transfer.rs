use super::Native;
use crate::{session::Session, tui::app::session_runtime::PromptRequest};
impl codetether_chat_submit::handoff::Transfer for Native<'_> {
    type Session = Session;
    type Request = PromptRequest;
    fn request_id(request: &PromptRequest) -> &str {
        &request.session.id
    }
    fn into_session(request: PromptRequest) -> Session {
        request.session
    }
    fn activate(&self, session_id: &str) -> bool {
        self.runtime.activate_steering(session_id)
    }
    async fn submit(&self, request: PromptRequest) -> Result<(), PromptRequest> {
        self.runtime.submit(request).await
    }
    fn clear_steering(&self) {
        self.runtime.clear_steering();
    }
    fn restore(&mut self, session: Session) {
        self.slot.restore(session);
    }
    fn status(&mut self, message: &str) {
        self.app.state.status = message.to_string();
    }
}
