//! Optional keep-last pass used during prompt-too-long retries.
use anyhow::Result;
use codetether_context_compression::{CompressContext, compress_messages_keep_last};
use codetether_provider_core::{Message, Provider};
use std::sync::Arc;
pub(super) async fn compress(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: &Arc<dyn Provider>,
    model: &str,
    force_keep_last: Option<usize>,
) -> Result<bool> {
    let forced = match force_keep_last {
        Some(keep_last) => {
            compress_messages_keep_last(
                messages,
                ctx,
                Arc::clone(provider),
                model,
                keep_last,
                "prompt_too_long_retry",
            )
            .await?
        }
        None => false,
    };
    Ok(forced)
}
