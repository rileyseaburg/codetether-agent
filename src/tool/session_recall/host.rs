//! Delegate indexed retrieval and answer synthesis to the application's backends.
use crate::provider::Provider;
use crate::rlm::{RlmConfig, RlmResult, RlmRouter};
use codetether_session_recall::{RecallHit, SearchOptions};
use std::{path::Path, sync::Arc};
pub struct ApplicationHost;
#[async_trait::async_trait]
impl codetether_session_recall::Host for ApplicationHost {
    async fn search(
        workspace: &Path,
        query: &str,
        options: SearchOptions<'_>,
    ) -> anyhow::Result<Vec<RecallHit>> {
        crate::session::index::recall::search::run(
            workspace,
            query,
            crate::session::index::recall::search::SearchOptions {
                session_id: options.session_id,
                excluded_session: options.excluded_session,
                limit: options.limit,
                minimum_score: options.minimum_score,
            },
        )
        .await
    }
    async fn synthesize(
        framed: &str,
        query: &str,
        provider: Arc<dyn Provider>,
        model: &str,
        config: &RlmConfig,
    ) -> anyhow::Result<RlmResult> {
        let auto_ctx = super::context::auto(provider, model, query);
        RlmRouter::auto_process(framed, auto_ctx, config).await
    }
}
