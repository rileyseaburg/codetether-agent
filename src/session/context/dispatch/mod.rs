//! Native specialization of shared context-policy execution.
use crate::provider::{Provider, ToolDefinition};
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use codetether_context_policy::{derivation::DerivePolicy, projection::helpers::DerivedContext};
use std::sync::Arc;
use tokio::sync::mpsc;
mod host;

/// Derive an ephemeral [`DerivedContext`] under a chosen [`DerivePolicy`].
///
/// Generalisation of [`crate::session::context::derive_context`] that accepts a policy selector.
/// [`DerivePolicy::Legacy`] delegates back to `derive_context` so the
/// two signatures co-exist without behaviour drift during rollout.
///
/// # Arguments
///
/// Same as [`crate::session::context::derive_context`], plus:
///
/// * `policy` — Which derivation strategy to run. See [`DerivePolicy`].
/// * `force_keep_last` — When `Some(n)`, bypass policy selection and fall
///   back to the legacy keep-last derivation used for prompt-too-long
///   recovery.
///
/// # Errors
///
/// Propagates any error from the underlying pipeline.
pub async fn derive_with_policy(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    policy: DerivePolicy,
    force_keep_last: Option<usize>,
) -> Result<DerivedContext> {
    let hooks = codetether_context_legacy::Hooks {
        apply: crate::session::helper::experimental::apply_all,
        repair: crate::session::helper::experimental::pairing::repair_orphans,
        resolver: crate::session::helper::rlm_background::resolve_model,
    };
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
