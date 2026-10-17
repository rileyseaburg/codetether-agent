//! Session/provider adapter for shared reset preparation and finalization.
use super::derive::derive_context;
use super::helpers::DerivedContext;
use crate::provider::ToolDefinition;
use crate::session::helper::token::estimate_request_tokens;
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use codetether_context_policy::derivation::reset::{self, ResetPreparation};
use std::sync::Arc;
use tokio::sync::mpsc;

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
    let origin_len = session.messages.len();
    let messages = session.messages.to_vec();
    match reset::prepare(messages, origin_len, threshold_tokens, |messages| {
        estimate_request_tokens(system_prompt, messages, tools)
    }) {
        ResetPreparation::Ready(context) => Ok(context),
        ResetPreparation::Legacy => {
            derive_context(
                session,
                provider,
                model,
                system_prompt,
                tools,
                event_tx,
                None,
            )
            .await
        }
        ResetPreparation::Summarize(plan) => {
            super::reset_finish::finish(session, plan, provider, model, event_tx).await
        }
    }
}
