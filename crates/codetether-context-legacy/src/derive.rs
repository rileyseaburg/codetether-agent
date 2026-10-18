//! Clone and transform history before assembling the derived context.
use super::compress_step::run_compression_step;
use anyhow::Result;
use codetether_context_policy::projection::helpers::DerivedContext;
use codetether_provider_core::ToolDefinition;
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;
/// Derive a compressed, repaired view without modifying the source transcript.
/// # Arguments
/// * `session` — Read-only transcript, identity and RLM settings.
/// * `provider` — Provider used by compression.
/// * `model` — Model determining the context budget.
/// * `system_prompt` — System text included in token estimates.
/// * `tools` — Available tool definitions.
/// * `event_tx` — Optional lifecycle event channel.
/// * `force_keep_last` — Optional forced prefix-compression tail size.
/// * `hooks` — Native transforms and background model resolver.
/// # Returns
/// Derived messages, full residency flags and compression provenance.
/// # Errors
/// Propagates failures from compression and budget enforcement.
/// # Examples
/// ```no_run
/// # async fn example<T: codetether_context_legacy::Source>(session: &T, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, hooks: codetether_context_legacy::Hooks) -> anyhow::Result<()> {
/// let derived = codetether_context_legacy::derive_context::<T, ()>(session, provider, "model", "", &[], None, None, hooks).await?;
/// assert_eq!(derived.origin_len, session.messages().len());
/// # Ok(()) }
/// ```
pub async fn derive_context<T: crate::Source, S>(
    session: &T,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    force_keep_last: Option<usize>,
    hooks: crate::Hooks,
) -> Result<DerivedContext> {
    let super::prepare::Prepared {
        origin_len,
        mut messages,
        ctx,
        step0,
        before,
    } = super::prepare::prepare(session, &provider, model, &hooks).await?;
    let step1 = run_compression_step(
        &mut messages,
        &ctx,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
        force_keep_last,
    )
    .await?;

    (hooks.repair)(&mut messages);

    Ok(super::finish::finish(
        origin_len, messages, before, step0, step1,
    ))
}
