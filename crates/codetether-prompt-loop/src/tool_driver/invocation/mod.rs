//! Persist-before-approval sequencing for an executable tool invocation.
//!
//! Implement [Host] with session-specific enrichment, persistence, approval,
//! execution, and rendering. [execute] preserves their order and warning metadata.
mod host;
use crate::Call;
use codetether_tool_outcome::Outcome;
pub use host::Host;

/// Apply approval policy, execute if allowed, and render the result.
///
/// # Arguments
/// * `host` — Existing session and tool-runtime operations.
/// * `call` — Normalized tool call with its original arguments and identifier.
/// # Returns
/// The rendered outcome, retaining approval warnings and execution metadata.
/// # Errors
/// Propagates pre-tool persistence failure before approval or execution begins.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_prompt_loop::tool_driver::invocation::Host, call: &codetether_prompt_loop::Call) -> anyhow::Result<()> {
/// let _outcome = codetether_prompt_loop::tool_driver::invocation::execute(host, call).await?;
/// # Ok(()) }
/// ```
pub async fn execute(host: &mut impl Host, call: &Call) -> anyhow::Result<Outcome> {
    let input = host.enrich(call);
    let started = host.before_tool().await?;
    let (input, blocked, warnings) = host.gate(call, input).await.into_checked_parts();
    let tuple = match blocked {
        Some(blocked) => blocked,
        None => host.approved(call, &input, started).await,
    };
    let tuple = codetether_tool_approval::annotate(tuple, warnings);
    Ok(host.render(call, &input, started, tuple).await)
}
