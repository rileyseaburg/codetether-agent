//! Native transcript bookkeeping and publication for shared steering inputs.
use crate::{provider::Message, session::Session};
impl codetether_session_steering::Transcript for Session {
    fn id(&self) -> &str {
        &self.id
    }
    fn add_human_message(&mut self, message: Message) {
        Session::add_human_message(self, message);
    }
    fn publish_user_prompt(&self, text: &str) {
        crate::session::helper::publish_user_prompt::publish(self, text, None);
    }
}
