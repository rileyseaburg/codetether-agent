//! Native specialization of shared context-policy execution.
use anyhow::Result;
use codetether_context_policy::{derivation::DerivePolicy, projection::helpers::DerivedContext};
use codetether_provider_core::ToolDefinition;
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Derive an ephemeral [`DerivedContext`] under a chosen [`DerivePolicy`].
///
/// Generalisation of [`crate::derive_context`] that accepts a policy selector.
/// [`DerivePolicy::Legacy`] delegates back to `derive_context` so the
/// two signatures co-exist without behaviour drift during rollout.
///
/// # Arguments
///
/// Same as [`crate::derive_context`], plus:
///
/// * `policy` — Which derivation strategy to run. See [`DerivePolicy`].
/// * `force_keep_last` — When `Some(n)`, bypass policy selection and fall
///   back to the legacy keep-last derivation used for prompt-too-long
///   recovery.
///
/// # Errors
///
/// Propagates any error from the underlying pipeline.
pub async fn derive_with_policy<S: codetether_context_dispatch::Source>(
    session: &S,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    policy: DerivePolicy,
    force_keep_last: Option<usize>,
) -> Result<DerivedContext> {
    let hooks = super::hooks::native();
    codetether_context_dispatch::derive_with_policy(
        session,
        provider,
        model,
        system_prompt,
        tools,
        event_tx,
        policy,
        force_keep_last,
        hooks,
    )
    .await
}
