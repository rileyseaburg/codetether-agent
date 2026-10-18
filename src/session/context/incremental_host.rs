//! Native transcript and prepared-index binding for incremental projection.
use crate::{
    provider::Message,
    session::{PageKind, Session, index::SummaryIndex},
};
use async_trait::async_trait;
#[async_trait]
impl codetether_incremental_derivation::Source for Session {
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn pages(&self) -> &[PageKind] {
        &self.pages
    }
    async fn prepared_index(&self) -> SummaryIndex {
        crate::session::index_produce::proactive::prepared_index(self).await
    }
}
