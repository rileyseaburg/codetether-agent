//! Public context-derivation entry point and final state header.
use crate::{Source, input::DispatchInputs};
use anyhow::Result;
use codetether_context_policy::{
    derivation::{DerivePolicy, dispatch::select},
    projection::helpers::DerivedContext,
};
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Derive an ephemeral [`DerivedContext`] under a chosen [`DerivePolicy`].
///
/// Generalisation of [`codetether_context_legacy::derive_context`] that accepts a policy selector.
/// [`DerivePolicy::Legacy`] delegates back to `derive_context` so the
/// two signatures co-exist without behaviour drift during rollout.
///
/// # Arguments
///
/// * `session` — Immutable inputs shared by the three strategies.
/// * `provider` — Provider used for model-assisted compression.
/// * `model` — Model determining applicable context limits.
/// * `system_prompt` — System instructions included in budget estimates.
/// * `tools` — Tool definitions included in estimates.
/// * `event_tx` — Optional compaction lifecycle event channel.
/// * `hooks` — Native transforms and background model resolution.
///
/// * `policy` — Which derivation strategy to run. See [`DerivePolicy`].
/// * `force_keep_last` — When `Some(n)`, bypass policy selection and fall
///   back to the legacy keep-last derivation used for prompt-too-long
///   recovery.
///
/// # Errors
///
/// Propagates any error from the underlying pipeline.
/// # Returns
/// The selected derived context with the final pinned-state header attached.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_context_dispatch::Source>(session: &H, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, hooks: codetether_context_legacy::Hooks) -> anyhow::Result<()> {
/// let _context = codetether_context_dispatch::derive_with_policy::<H, ()>(session, provider, "model", "", &[], None, codetether_context_policy::derivation::DerivePolicy::Legacy, None, hooks).await?;
/// # Ok(()) }
/// ```
pub async fn derive_with_policy<H: Source, S>(
    session: &H,
    provider: Arc<dyn Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    policy: DerivePolicy,
    force_keep_last: Option<usize>,
    hooks: codetether_context_legacy::Hooks,
) -> Result<DerivedContext> {
    let input = DispatchInputs {
        session,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
        force_keep_last,
        hooks,
    };
    let mut derived = super::execute::run(input, select(policy, force_keep_last)).await?;
    super::state_header::prepend_state_header(session, &mut derived.messages);
    Ok(derived)
}
