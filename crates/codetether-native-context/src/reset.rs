//! Native bindings for shared reset-policy context derivation.
use anyhow::Result;
use codetether_context_policy::projection::helpers::DerivedContext;
use codetether_provider_core::ToolDefinition;
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// [`DerivePolicy::Reset`](codetether_context_policy::derivation::DerivePolicy::Reset) implementation.
///
/// When the token estimate exceeds `threshold_tokens`, summarise
/// everything older than the active task tail via the RLM router and
/// return `[summary_message, active_task_tail...]`. The tail starts at
/// the latest substantive user turn, so short follow-ups like "continue"
/// and "status?" do not orphan the real task request. Under threshold,
/// return the full clone verbatim.
#[allow(clippy::too_many_arguments)]
pub async fn derive_reset<S: codetether_reset_derivation::Source>(
    session: &S,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    threshold_tokens: usize,
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
) -> Result<DerivedContext> {
    let hooks = super::hooks::native();
    codetether_reset_derivation::derive_reset(
        session,
        provider,
        model,
        system_prompt,
        tools,
        threshold_tokens,
        event_tx,
        hooks,
    )
    .await
}
