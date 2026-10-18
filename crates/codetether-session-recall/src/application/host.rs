//! Delegate indexed search and execute the original RLM synthesis request.
use super::{ApplicationHost, Search, context};
use crate::{Host, RecallHit, SearchOptions};
use codetether_provider_core::Provider;
use codetether_rlm::{RlmConfig, RlmResult};
use codetether_rlm_routing::{Bus, RlmRouter};
use std::{path::Path, sync::Arc};
#[async_trait::async_trait]
impl<B: Bus, S: Search> Host for ApplicationHost<B, S> {
    async fn search(
        workspace: &Path,
        query: &str,
        options: SearchOptions<'_>,
    ) -> anyhow::Result<Vec<RecallHit>> {
        S::search(workspace, query, options).await
    }
    async fn synthesize(
        framed: &str,
        query: &str,
        provider: Arc<dyn Provider>,
        model: &str,
        config: &RlmConfig,
    ) -> anyhow::Result<RlmResult> {
        let auto_ctx = context::auto::<B>(provider, model, query);
        RlmRouter::auto_process(framed, auto_ctx, config).await
    }
}
