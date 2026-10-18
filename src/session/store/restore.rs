//! Native construction and mutable fields for shared indexed restoration.
use crate::{
    agent::ToolUse,
    provider::Message,
    session::{Session, index::SummaryIndex, pages::PageKind, tracked::TrackedVec},
};
use codetether_session_store::Restore;
impl Restore for Session {
    fn decode_header(header: &str) -> anyhow::Result<Self> {
        super::header::decode(header)
    }
    fn messages_mut(&mut self) -> &mut TrackedVec<Message> {
        &mut self.messages
    }
    fn pages_mut(&mut self) -> &mut TrackedVec<PageKind> {
        &mut self.pages
    }
    fn tools_mut(&mut self) -> &mut TrackedVec<ToolUse> {
        &mut self.tool_uses
    }
    fn set_summary_index(&mut self, index: SummaryIndex) {
        self.summary_index = index;
    }
    fn normalize_sidecars(&mut self) {
        Session::normalize_sidecars(self);
    }
}
