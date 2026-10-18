//! Native transcript operations for shared session orchestration.
use crate::{provider::Message, session::Session};
impl codetether_session_orchestration::Session for Session {
    fn add_message(&mut self, message: Message) {
        Session::add_message(self, message);
    }
    fn messages(&self) -> &[Message] {
        &self.messages
    }
}
