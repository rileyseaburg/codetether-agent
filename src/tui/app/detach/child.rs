//! Native session field bindings for detached snapshots.
use crate::{provider::Message, session::Session};
impl codetether_session_fork::detached::Session for Session {
    fn replace_transcript(&mut self, messages: Vec<Message>) {
        self.messages = messages.into();
        self.pages = crate::session::pages::classify_all(&self.messages).into();
    }
    fn set_detached_agent(&mut self) {
        self.agent = "detached".to_string();
    }
    fn attach_bus_if_missing(&mut self) {
        self.attach_global_bus_if_missing();
    }
}
#[cfg(test)]
use codetether_session_fork::detached::tail;
#[cfg(test)]
#[path = "child_tests.rs"]
mod tests;
