//! Native session field and header bindings for shared delta persistence.
use crate::{
    agent::ToolUse,
    provider::Message,
    session::{Session, pages::PageKind, tracked::TrackedVec},
};
use codetether_session_store::{Snapshot, State};
impl Snapshot for Session {
    type Tool = ToolUse;
    fn id(&self) -> &str {
        &self.id
    }
    fn state(&self) -> &State {
        &self.storage
    }
    fn messages(&self) -> &TrackedVec<Message> {
        &self.messages
    }
    fn pages(&self) -> &TrackedVec<PageKind> {
        &self.pages
    }
    fn tools(&self) -> &TrackedVec<ToolUse> {
        &self.tool_uses
    }
    fn encode_header(&self) -> anyhow::Result<String> {
        super::header::encode(self)
    }
}
