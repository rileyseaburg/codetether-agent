//! Native bindings for shared reset-policy context derivation.
use super::helpers::DerivedContext;
use crate::provider::ToolDefinition;
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
#[path = "reset_host.rs"]
mod host;

/// [`DerivePolicy::Reset`](crate::session::derive_policy::DerivePolicy::Reset) implementation.
///
/// When the token estimate exceeds `threshold_tokens`, summarise
/// everything older than the active task tail via the RLM router and
/// return `[summary_message, active_task_tail...]`. The tail starts at
/// the latest substantive user turn, so short follow-ups like "continue"
/// and "status?" do not orphan the real task request. Under threshold,
/// return the full clone verbatim.
#[allow(clippy::too_many_arguments)]
pub(super) async fn derive_reset(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    threshold_tokens: usize,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<DerivedContext> {
    let hooks = codetether_context_legacy::Hooks {
        apply: crate::session::helper::experimental::apply_all,
        repair: crate::session::helper::experimental::pairing::repair_orphans,
        resolver: crate::session::helper::rlm_background::resolve_model,
    };
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
