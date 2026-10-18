//! Native metadata and state operations for shared session imports.
use crate::session::{Session, index::SummaryIndex};
use codetether_session_store::{State, legacy::Legacy};
impl Legacy for Session {
    fn legacy_header(&self) -> anyhow::Result<String> {
        super::header::legacy(self)
    }
    fn summary_index(&self) -> &SummaryIndex {
        &self.summary_index
    }
    fn is_at_least_as_new_as(&self, incoming: &Self) -> bool {
        self.updated_at >= incoming.updated_at
    }
    fn replace_state(&mut self, state: State) {
        self.storage = state;
    }
}
