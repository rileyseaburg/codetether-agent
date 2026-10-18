//! Prepared-index incremental derivation with the native entry-point signature.
use anyhow::Result;
use codetether_context_policy::projection::helpers::DerivedContext;
use codetether_provider_core::ToolDefinition;
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;
/// [`DerivePolicy::Incremental`](codetether_context_policy::derivation::DerivePolicy::Incremental)
/// implementation.
pub async fn derive_incremental<S: codetether_incremental_derivation::Source>(
    session: &S,
    _provider: Arc<dyn codetether_provider_core::Provider>,
    _model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
    _event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
) -> Result<DerivedContext> {
    codetether_incremental_derivation::derive_incremental(
        session,
        system_prompt,
        tools,
        budget_tokens,
    )
    .await
}
