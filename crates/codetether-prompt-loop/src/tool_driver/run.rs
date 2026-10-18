//! Single-call sequencing with early returns for discovery and policy rejection.
use super::Host;
use crate::Call;
/// Run a normalized call through policy, execution and publication.
/// # Arguments
/// * `host` — Native policy and execution bindings.
/// * `step` — Current provider step.
/// * `call` — Owned normalized tool call.
/// # Returns
/// Whether code-search recovery requests a retry after the completed call.
/// # Errors
/// Propagates discovery, execution, refresh and publication errors.
/// # Examples
/// ```no_run
/// # async fn execute<H: codetether_prompt_loop::tool_driver::Host>(host: &mut H, call: codetether_prompt_loop::Call) -> anyhow::Result<()> {
/// let _retry = codetether_prompt_loop::tool_driver::run(host, 1, call).await?;
/// # Ok(()) }
/// ```
pub async fn run(host: &mut impl Host, step: usize, call: Call) -> anyhow::Result<bool> {
    host.start(&call).await;
    if call.name == "list_tools" {
        return host.discover(&call).await;
    }
    host.request(step, &call);
    if let Some(reason) = host.blocked(&call) {
        host.rejected(&call, format!("Error: {reason}")).await;
        return Ok(false);
    }
    let outcome = host.execute(&call).await?;
    host.progress()
        .decline_guard
        .record(&call.name, &call.input, outcome.metadata.as_ref());
    host.refresh(&call, outcome.success).await?;
    host.publish(step, &call, outcome).await?;
    Ok(host.codesearch_guard(step))
}
