//! Native background-model resolver binding for oversized-message compression.
use super::context::CompressContext;
use crate::provider::{Message, Provider};
use anyhow::Result;
use std::sync::Arc;
pub(crate) async fn compress_last_message_if_oversized(
    messages: &mut [Message],
    ctx: &CompressContext,
    provider: Arc<dyn Provider>,
    model: &str,
) -> Result<bool> {
    codetether_context_compression::compress_last_message_if_oversized(
        messages,
        ctx,
        provider,
        model,
        crate::session::helper::rlm_background::Background::resolve_model,
    )
    .await
}
