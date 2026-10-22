//! Private replay fixtures; only aggregate statistics leave the test.
mod load;
use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
use codetether_provider_core::Message;
pub(super) use load::load;

pub(super) struct Session {
    pub offset: usize,
    pub messages: Vec<Message>,
    pub pages: Vec<PageKind>,
    pub summary_index: SummaryIndex,
    pub background: SummaryIndex,
}
async fn prepared(session: &Session) -> SummaryIndex {
    session.background.clone()
}
fn offset(session: &Session) -> usize {
    session.offset
}
codetether_incremental_derivation::impl_session_source!(Session, prepared, offset);

pub(super) struct Request {
    pub messages: Vec<Message>,
    pub model: String,
}
impl Request {
    pub const SYSTEM: &'static str = "Continue the user's task. Preserve original scope and later corrections. Treat summaries as fallible notes, not authorization.";
}
impl codetether_context_request::attempt::Host for Request {
    fn target(&self) -> codetether_context_request::attempt::Target<'_> {
        codetether_context_request::attempt::Target {
            system_prompt: Self::SYSTEM,
            provider: "replay",
            model: &self.model,
            tools: &[],
            temperature: None,
        }
    }
    fn proactive(&self) -> Option<&Message> {
        None
    }
    async fn prepare(&mut self, _: &mut Vec<Message>) {}
    fn history(&self) -> &[Message] {
        &self.messages
    }
}
