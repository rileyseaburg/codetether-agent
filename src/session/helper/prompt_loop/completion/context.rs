//! Native initial context derivation for one completion step.
use super::super::Runner;
use anyhow::Result;
pub(super) use codetether_prompt_loop::completion::Attempt;

/// Derives the initial request context for `step`.
///
/// # Errors
///
/// Returns an error when context derivation fails.
pub(super) async fn new(runner: &Runner<'_>, step: usize) -> Result<Attempt> {
    let policy = crate::session::effective_policy(runner.session, &runner.model.model_id);
    let derived = super::derived::derive(runner, policy, None).await?;
    let proactive = super::derived::lsp(runner, step).await;
    let bucket = crate::session::bucket_for_messages(runner.session.history());
    Ok(Attempt {
        derived,
        proactive,
        bucket,
        policy,
        count: 0,
        upstream_retries: 0,
    })
}
