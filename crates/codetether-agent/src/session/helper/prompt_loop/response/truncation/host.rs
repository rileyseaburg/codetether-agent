//! Native transcript, output and event-stream bindings.
use crate::{
    provider::Message,
    session::{Session, SessionEvent},
};
use tokio::sync::mpsc;
pub(super) struct Native<'a> {
    pub session: &'a mut Session,
    pub output: &'a mut String,
    pub events: Option<&'a mpsc::Sender<SessionEvent>>,
}
impl codetether_prompt_loop::repair::Host for Native<'_> {
    type Snapshot = Session;
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent>> {
        self.events
    }
    fn add_message(&mut self, message: Message) {
        self.session.add_message(message);
    }
    fn clear_output(&mut self) {
        self.output.clear();
    }
}
