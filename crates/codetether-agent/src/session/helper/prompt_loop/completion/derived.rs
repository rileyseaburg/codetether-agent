//! Derivation of provider history and proactive LSP context.

use super::super::Runner;
use crate::provider::Message;
use crate::session::{DerivePolicy, DerivedContext, derive_with_policy};
use anyhow::Result;

/// Derives the bounded message history for a provider attempt.
///
/// # Errors
///
/// Returns an error when context derivation or compaction fails.
pub(super) async fn derive(
    runner: &Runner<'_>,
    policy: DerivePolicy,
    keep: Option<usize>,
) -> Result<DerivedContext> {
    derive_with_policy(
        runner.inner.session,
        runner.inner.model.provider.clone(),
        &runner.inner.model.model_id,
        &runner.inner.model.system_prompt,
        &runner.inner.model.advertised,
        runner.inner.events.as_ref(),
        policy,
        keep,
    )
    .await
}

/// Builds optional proactive language-server context for `step`.
pub(super) async fn lsp(runner: &Runner<'_>, step: usize) -> Option<Message> {
    super::super::super::router::build_proactive_lsp_context_message(
        &runner.inner.model.provider_name,
        step,
        &runner.inner.model.registry,
        &runner.inner.session.messages,
        &runner.inner.workspace.cwd,
    )
    .await
}
