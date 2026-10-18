//! Session and prepared-index access for shared incremental context derivation.
use super::helpers::DerivedContext;
use crate::provider::ToolDefinition;
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
#[cfg(test)]
#[path = "incremental_pack.rs"]
mod pack;
#[cfg(test)]
#[path = "incremental_required.rs"]
mod required;

codetether_incremental_derivation::impl_session_source!(
    Session,
    crate::session::index_produce::proactive::prepared_index
);

/// [`DerivePolicy::Incremental`](crate::session::derive_policy::DerivePolicy::Incremental)
/// implementation.
pub(super) async fn derive_incremental(
    session: &Session,
    _provider: Arc<dyn crate::provider::Provider>,
    _model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    budget_tokens: usize,
    _event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<DerivedContext> {
    codetether_incremental_derivation::derive_incremental(
        session,
        system_prompt,
        tools,
        budget_tokens,
    )
    .await
}
