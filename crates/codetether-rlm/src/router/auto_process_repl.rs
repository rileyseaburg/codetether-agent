//! Run router auto-processing with the shared in-memory REPL host.
use super::{CrateAutoProcessContext, ReplHost};
use crate::{
    RlmConfig, RlmResult,
    repl::{ReplRuntime, RlmRepl},
};
/// Process output with the standard in-memory context and tool dispatcher.
///
/// # Arguments
/// * `output` - Text to analyze.
/// * `ctx` - Provider and event adapters for the run.
/// * `config` - Routing and iteration settings.
/// # Returns
/// Router output and its execution statistics.
/// # Errors
/// Propagates errors from the router's auto-processing implementation.
/// # Examples
/// ```no_run
/// # async fn example(ctx: codetether_rlm::CrateAutoProcessContext<'_>) -> anyhow::Result<()> {
/// let result = codetether_rlm::router::auto_process_with_repl(
///     "context", ctx, &codetether_rlm::RlmConfig::default(),
/// ).await?;
/// let _ = result.processed;
/// # Ok(()) }
/// ```
pub async fn auto_process_with_repl(
    output: &str,
    ctx: CrateAutoProcessContext<'_>,
    config: &RlmConfig,
) -> anyhow::Result<RlmResult> {
    let mut repl = RlmRepl::new(output.to_string(), ReplRuntime::Rust);
    let mut host = ReplHost(&mut repl);
    super::auto_process(output, ctx, config, &mut host).await
}
