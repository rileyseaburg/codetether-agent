//! Adapt the application's summary cache and storage without changing ownership.
use crate::provider::Provider;
use crate::session::{
    Session,
    index::{SummaryIndex, SummaryNode, SummaryRange},
};
use std::sync::Arc;
#[async_trait::async_trait]
impl codetether_context_summarize::SummarySource for Session {
    type Prepared = super::prepared::Prepared;
    fn summary_index(&self) -> &SummaryIndex {
        &self.summary_index
    }
    fn summary_index_mut(&mut self) -> &mut SummaryIndex {
        &mut self.summary_index
    }
    fn prepare(&self) -> Self::Prepared {
        super::prepared::capture(self)
    }
    async fn produce(
        prepared: Self::Prepared,
        range: SummaryRange,
        target: usize,
        provider: Arc<dyn Provider>,
        model: &str,
    ) -> anyhow::Result<SummaryNode> {
        codetether_context_summarize::production::produce::<crate::session::SessionBus>(
            prepared,
            range,
            target,
            provider,
            model,
            crate::session::helper::rlm_background::Background::resolve_model,
        )
        .await
    }
    async fn persist_summary(
        &mut self,
        range: SummaryRange,
        node: &SummaryNode,
    ) -> anyhow::Result<()> {
        crate::session::store::summaries::put_snapshot(self, range, node).await
    }
}
