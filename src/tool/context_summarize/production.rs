//! Delegate summary generation to the existing RLM producer.
use super::prepared::Prepared;
use crate::provider::Provider;
use crate::session::index::{Granularity, SummaryNode, SummaryRange};
use crate::session::index_produce::{SummaryObservability, produce_summary};
use std::sync::Arc;
pub(super) async fn produce(
    prepared: Prepared,
    range: SummaryRange,
    target: usize,
    provider: Arc<dyn Provider>,
    model: &str,
) -> anyhow::Result<SummaryNode> {
    let Prepared {
        messages,
        generation,
        rlm,
        session_id,
        sub_provider,
        sub_model,
    } = prepared;
    produce_summary(
        &messages,
        range,
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
    .await
}
