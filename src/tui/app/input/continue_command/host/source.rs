use super::Native;
use crate::{provider::Message, session::Session};
impl codetether_chat_submit::resume::Source for Native<'_> {
    type Session = Session;
    fn inflight(&self) -> Option<String> {
        self.app.state.main_inflight_prompt.clone()
    }
    fn session(&self) -> Option<&Session> {
        self.slot.borrow()
    }
    fn durable_prompt(session: &Session) -> Option<String> {
        crate::session::tasks::runtime::resume_prompt(&session.id)
    }
    fn messages(session: &Session) -> &[Message] {
        &session.messages
    }
    fn is_processing(&self) -> bool {
        self.app.state.processing
    }
}
