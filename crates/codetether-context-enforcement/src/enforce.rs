//! Adaptive context-window enforcement: budget gate + cascade entry.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use codetether_provider_core::ToolDefinition;
use codetether_session_events::SessionEvent;

use super::enforce_budget::check_budget;
use super::enforce_run::cascade_or_truncate;
use codetether_context_compression::CompressContext;

/// Ensure the estimated request token cost of `messages` fits within the
/// model's safety budget, in-place.
///
/// Operates on a supplied buffer without borrowing native session state.
///
/// # Errors
///
/// Returns `Err` only if an underlying RLM pass errors in a way the
/// recovery cascade cannot absorb. Terminal truncation is infallible.
/// # Arguments
/// * `messages` — Buffer or projection to fit within the request budget.
/// * `ctx` — Session identity and RLM configuration snapshot.
/// * `provider` — Provider used for foreground summarization.
/// * `model` — Model determining the usable context budget.
/// * `system_prompt` — System text included in estimates.
/// * `tools` — Tool definitions included in estimates.
/// * `event_tx` — Optional compaction lifecycle event destination.
/// # Returns
/// Success after the cascade and terminal fallback finish.
/// # Examples
/// ```no_run
/// # async fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let ctx = codetether_context_compression::CompressContext::new("session", &Default::default());
/// let mut messages = Vec::new();
/// codetether_context_enforcement::enforce_on_messages::<()>(&mut messages, &ctx, provider, "model", "", &[], None).await.unwrap();
/// assert!(messages.is_empty());
/// # }
/// ```
pub async fn enforce_on_messages<S>(
    messages: &mut Vec<codetether_provider_core::Message>,
    ctx: &CompressContext,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
) -> Result<()> {
    let check = check_budget(messages, ctx, model, system_prompt, tools);
    let Some(trigger_reason) = check.trigger_reason else {
        return Ok(());
    };

    let trace_id = super::start::start(event_tx, trigger_reason, &check).await;

    cascade_or_truncate(
        messages,
        ctx,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
        trace_id,
        trigger_reason,
        &check,
    )
    .await
}
