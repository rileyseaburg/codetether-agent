//! Native background-model resolver binding for oversized-message compression.
use anyhow::Result;
use codetether_context_compression::CompressContext;
use codetether_provider_core::{Message, Provider};
use std::sync::Arc;
/// Compress an oversized last message using native background-model policy.
/// # Arguments
/// * messages - Mutable transcript window.
/// * ctx - Compression settings and session identity.
/// * provider - Fallback model provider.
/// * model - Fallback model name.
/// # Returns
/// True when the last message was compressed.
/// # Errors
/// Propagates errors from shared oversized-message compression.
/// # Examples
/// ```no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, messages: &mut [codetether_provider_core::Message], ctx: &codetether_context_compression::CompressContext) -> anyhow::Result<()> {
/// let changed = codetether_native_context::compress_last_message_if_oversized(messages, ctx, provider, "model").await?;
/// # Ok(()) }
/// ```
pub async fn compress_last_message_if_oversized(
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
        super::hooks::native().resolver,
    )
    .await
}
