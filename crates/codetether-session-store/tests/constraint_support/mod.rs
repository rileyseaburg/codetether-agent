use codetether_context_policy::{pages::PageKind, summary_index::SummaryIndex};
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_session_store::{
    Restore, Snapshot, State,
    window::{WorkingSet, append},
};
use codetether_tracked_vec::TrackedVec;

#[derive(Default)]
pub struct Session {
    pub state: State,
    pub messages: TrackedVec<Message>,
    pub pages: TrackedVec<PageKind>,
    pub tools: TrackedVec<()>,
    pub summaries: SummaryIndex,
}
pub fn message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
impl Snapshot for Session {
    type Tool = ();
    fn id(&self) -> &str {
        "constraints-session"
    }
    fn state(&self) -> &State {
        &self.state
    }
    fn messages(&self) -> &TrackedVec<Message> {
        &self.messages
    }
    fn pages(&self) -> &TrackedVec<PageKind> {
        &self.pages
    }
    fn tools(&self) -> &TrackedVec<()> {
        &self.tools
    }
    fn encode_header(&self) -> anyhow::Result<String> {
        Ok("{}".into())
    }
}
impl Restore for Session {
    fn decode_header(_: &str) -> anyhow::Result<Self> {
        Ok(Self::default())
    }
    fn messages_mut(&mut self) -> &mut TrackedVec<Message> {
        &mut self.messages
    }
    fn pages_mut(&mut self) -> &mut TrackedVec<PageKind> {
        &mut self.pages
    }
    fn tools_mut(&mut self) -> &mut TrackedVec<()> {
        &mut self.tools
    }
    fn set_summary_index(&mut self, index: SummaryIndex) {
        self.summaries = index;
    }
    fn normalize_sidecars(&mut self) {}
}
impl Session {
    pub fn append(&mut self, text: &str) {
        append(
            WorkingSet {
                id: "constraints-session",
                state: &self.state,
                messages: &mut self.messages,
                pages: &mut self.pages,
                tools: &mut self.tools,
                summaries: &mut self.summaries,
            },
            message(text),
        );
    }
}
