//! Session/provider adapter for shared reset preparation and finalization.
use anyhow::Result;
use codetether_context_legacy::derive_context;
use codetether_context_policy::derivation::reset::ResetPreparation;
use codetether_context_policy::projection::helpers::DerivedContext;
use codetether_provider_core::ToolDefinition;
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// [`codetether_context_policy::derivation::DerivePolicy::Reset`] implementation.
///
/// When the token estimate exceeds `threshold_tokens`, summarise
/// everything older than the active task tail via the RLM router and
/// return `[summary_message, active_task_tail...]`. The tail starts at
/// the latest substantive user turn, so short follow-ups like "continue"
/// and "status?" do not orphan the real task request. Under threshold,
/// return the full clone verbatim.
/// # Arguments
/// * `session` — Read-only transcript, RLM settings and native subcall cache.
/// * `provider` — Caller provider used by legacy fallback or summaries.
/// * `model` — Caller model identifier.
/// * `system_prompt` — System text included in budget estimates.
/// * `tools` — Tool definitions included in estimates.
/// * `threshold_tokens` — Reset threshold used by shared preparation.
/// * `event_tx` — Optional compaction event channel.
/// * `hooks` — Native transforms and background model resolver.
/// # Returns
/// A cloned, legacy-compressed or rebuilt derived context.
/// # Errors
/// Propagates legacy compression or summary preparation errors.
/// # Examples
/// ```no_run
/// # async fn example<T: codetether_reset_derivation::Source>(session: &T, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, hooks: codetether_context_legacy::Hooks) -> anyhow::Result<()> {
/// let context = codetether_reset_derivation::derive_reset::<T, ()>(session, provider, "model", "", &[], 128_000, None, hooks).await?;
/// assert_eq!(context.origin_len, session.messages().len());
/// # Ok(()) }
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn derive_reset<T: crate::Source, S>(
    session: &T,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    threshold_tokens: usize,
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    hooks: codetether_context_legacy::Hooks,
) -> Result<DerivedContext> {
    match super::prepare::prepare(session, threshold_tokens, system_prompt, tools) {
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
                hooks,
            )
            .await
        }
        ResetPreparation::Summarize(plan) => {
            super::reset_finish::finish(session, plan, provider, model, event_tx, hooks.resolver)
                .await
        }
    }
}
