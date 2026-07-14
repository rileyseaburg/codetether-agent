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
        runner.session,
        runner.model.provider.clone(),
        &runner.model.model_id,
        &runner.model.system_prompt,
        &runner.model.advertised,
        runner.events.as_ref(),
        policy,
        keep,
    )
    .await
}

/// Builds optional proactive language-server context for `step`.
pub(super) async fn lsp(runner: &Runner<'_>, step: usize) -> Option<Message> {
    super::super::super::router::build_proactive_lsp_context_message(
        &runner.model.provider_name,
        step,
        &runner.model.registry,
        &runner.session.messages,
        &runner.workspace.cwd,
    )
    .await
}
