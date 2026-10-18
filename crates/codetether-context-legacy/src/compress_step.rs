//! Coordinate optional forced compression and mandatory budget enforcement.
use anyhow::Result;
use codetether_context_compression::CompressContext;
use codetether_context_enforcement::enforce_on_messages;
use codetether_context_policy::projection::helpers::messages_len_changed;
use codetether_provider_core::{Message, Provider, ToolDefinition};
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Run either the adaptive budget cascade or a forced keep-last
/// compression pass, returning whether compression fired.
pub(super) async fn run_compression_step<S>(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    force_keep_last: Option<usize>,
) -> Result<bool> {
    let before = messages.len();
    let forced = super::forced::compress(messages, ctx, &provider, model, force_keep_last).await?;
    // A forced pass can leave a short-but-oversized history over budget,
    // so the normal cascade + terminal truncation must always follow it.
    enforce_on_messages(
        messages,
        ctx,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
    )
    .await?;
    Ok(forced || messages_len_changed(before, messages))
}
