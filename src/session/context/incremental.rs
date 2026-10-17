//! Session and prepared-index access for shared incremental context derivation.
use super::helpers::DerivedContext;
use crate::provider::ToolDefinition;
use crate::session::helper::token::{estimate_request_tokens, estimate_tokens_for_messages};
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use codetether_context_policy::derivation::incremental::{finish, prepare};
use std::sync::Arc;
use tokio::sync::mpsc;
#[cfg(test)]
#[path = "incremental_pack.rs"]
mod pack;
#[path = "incremental_required.rs"]
mod required;

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
    let origin_len = session.messages.len();
    let mut clone = session.messages.to_vec();
    if let Some(ctx) = super::incremental_below_budget::try_pass_through(
        &mut clone,
        system_prompt,
        tools,
        budget_tokens,
        origin_len,
    ) {
        return Ok(ctx);
    }

    let selection = prepare(
        clone,
        origin_len,
        budget_tokens,
        |messages| estimate_request_tokens(system_prompt, messages, tools),
        estimate_tokens_for_messages,
        |keep, costs, recent, budget| required::seed(session, keep, costs, recent, budget),
    );
    let prepared = crate::session::index_produce::proactive::prepared_index(session).await;
    Ok(finish(
        selection,
        &prepared,
        |messages| estimate_request_tokens(system_prompt, messages, tools),
        estimate_tokens_for_messages,
    ))
}
