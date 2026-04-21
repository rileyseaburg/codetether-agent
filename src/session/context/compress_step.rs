//! Compression step: either adaptive enforce or forced keep-last.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::{Message, Provider, ToolDefinition};
use crate::session::SessionEvent;
use crate::session::helper::compression::{
    CompressContext, compress_messages_keep_last, enforce_on_messages,
};

use super::helpers::messages_len_changed;

/// Run either the adaptive budget cascade or a forced keep-last
/// compression pass, returning whether compression fired.
pub(super) async fn run_compression_step(
    messages: &mut Vec<Message>,
    ctx: &CompressContext,
    provider: Arc<dyn Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    force_keep_last: Option<usize>,
) -> Result<bool> {
    let before = messages.len();
    match force_keep_last {
        Some(keep_last) => {
            compress_messages_keep_last(
                messages,
                ctx,
                Arc::clone(&provider),
                model,
                keep_last,
                "prompt_too_long_retry",
            )
            .await
        }
        None => {
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
            Ok(messages_len_changed(before, messages))
        }
    }
}
