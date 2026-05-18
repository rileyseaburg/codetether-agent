//! On-demand summary generation for `context_summarize`.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::Provider;
use crate::session::Session;
use crate::session::index::{Granularity, SummaryNode, SummaryRange};
use crate::session::index_produce::{SummaryObservability, produce_summary};

/// Generate, cache, and persist a missing summary.
pub async fn produce_cached(
    session: &mut Session,
    range: SummaryRange,
    target: usize,
    provider: Arc<dyn Provider>,
    model: &str,
) -> Result<SummaryNode> {
    let messages = session.messages.clone();
    let generation = session.summary_index.generation();
    let rlm = session.metadata.rlm.clone();
    let session_id = session.id.clone();
    let sub_provider = session.metadata.subcall_provider.clone();
    let sub_model = session.metadata.subcall_model_name.clone();
    let node = session
        .summary_index
        .summary_for(range, |r| {
            produce_summary(
                &messages,
                r,
                target,
                Granularity::Phase,
                generation,
                provider,
                model,
                &rlm,
                &session_id,
                sub_provider,
                sub_model,
                SummaryObservability::default(),
            )
        })
        .await?;
    session.save().await?;
    Ok(node)
}
