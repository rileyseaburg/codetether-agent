//! Native event specialization of shared context-budget enforcement.
use super::context::CompressContext;
use crate::provider::ToolDefinition;
use crate::session::SessionEvent;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
pub(crate) async fn enforce_on_messages(
    messages: &mut Vec<crate::provider::Message>,
    ctx: &CompressContext,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<()> {
    codetether_context_enforcement::enforce_on_messages(
        messages,
        ctx,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
    )
    .await
}
