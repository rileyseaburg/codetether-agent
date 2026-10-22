use super::State;
use codetether_prompt_loop::{Call, tool_publication::Host};
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_session_events::SessionEvent;
use codetether_tool_outcome::Outcome;
use tokio::sync::mpsc;
impl Host for State {
    type Snapshot = ();
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent<()>>> {
        None
    }
    fn broadcast(&self, _: usize, _: &Call, _: &Outcome) {}
    fn archive(&self, _: &Call, _: &Outcome) {}
    fn route(&self, _: &Call, content: &str) -> String {
        content.into()
    }
    fn append(&mut self, message: Message) {
        self.messages.push(message);
    }
    fn append_approval_feedback(&mut self, id: &str, feedback: String) {
        assert_eq!(id, "denied-call");
        self.messages.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: feedback }],
        });
    }
    async fn persist(&mut self) -> anyhow::Result<()> {
        assert_eq!(self.messages.len(), if self.spoof { 1 } else { 2 });
        self.persisted = true;
        Ok(())
    }
    fn record_codesearch(&mut self, _: bool) {}
}
