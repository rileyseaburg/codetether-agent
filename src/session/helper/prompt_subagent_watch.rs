//! Session bindings for shared child-result notice delivery.
use crate::{provider::Message, session::Session};
pub(in crate::session::helper) use codetether_agent_messages::notice::SubAgentWatch;
impl codetether_agent_messages::notice::Host for Session {
    fn session_id(&self) -> &str {
        &self.id
    }
    fn history(&self) -> &[Message] {
        Session::history(self)
    }
    fn add_delegated_message(&mut self, message: Message) {
        Session::add_delegated_message(self, message);
    }
}
