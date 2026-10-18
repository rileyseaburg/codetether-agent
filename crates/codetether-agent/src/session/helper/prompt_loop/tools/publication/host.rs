//! Native services for shared tool-result publication.
use super::super::super::Runner;
use crate::{
    provider::Message,
    session::{Session, SessionEvent},
};
use codetether_prompt_loop::Call;
use codetether_tool_outcome::Outcome;
use tokio::sync::mpsc;
impl codetether_prompt_loop::tool_publication::Host for Runner<'_> {
    type Snapshot = Session;
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent>> {
        self.inner.events.as_ref()
    }
    fn broadcast(&self, step: usize, call: &Call, outcome: &Outcome) {
        super::super::bus::response(self, step, call, outcome);
    }
    fn archive(&self, call: &Call, outcome: &Outcome) {
        super::super::archive::write(self, call, outcome);
    }
    fn route(&self, call: &Call, content: &str) -> String {
        super::super::rlm::route(self, call, content)
    }
    fn append(&mut self, message: Message) {
        self.inner.session.add_message(message);
    }
    async fn persist(&mut self) -> anyhow::Result<()> {
        super::super::super::super::persist::after_tool(self.inner.session).await
    }
    fn record_codesearch(&mut self, missed: bool) {
        super::super::codesearch::record(self, missed);
    }
}
