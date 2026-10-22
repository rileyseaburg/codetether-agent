//! Ordered delivery and persistence of a completed executable result.
use super::Host;
use crate::Call;
use codetether_tool_outcome::Outcome;
/// Publish a completed result, append its routed content, then persist it.
///
/// # Arguments
/// `host` supplies destinations; `step` and `call` identify the execution;
/// `outcome` supplies its output, metadata, status and timing.
///
/// # Returns
/// Returns success after persistence and code-search accounting.
///
/// # Errors
/// Propagates persistence errors before updating code-search accounting.
///
/// # Examples
/// ```no_run
/// # async fn publish(host: &mut impl codetether_prompt_loop::tool_publication::Host,
/// # call: &codetether_prompt_loop::Call, outcome: codetether_tool_outcome::Outcome)
/// # -> anyhow::Result<()> {
/// codetether_prompt_loop::tool_publication::complete(host, 1, call, outcome).await?;
/// # Ok(()) }
/// ```
pub async fn complete(
    host: &mut impl Host,
    step: usize,
    call: &Call,
    outcome: Outcome,
) -> anyhow::Result<()> {
    host.broadcast(step, call, &outcome);
    host.archive(call, &outcome);
    if let Some(tx) = host.events() {
        codetether_tool_reporting::send_metadata(
            tx,
            &call.id,
            &call.name,
            outcome.metadata.as_ref(),
        )
        .await;
        codetether_tool_reporting::complete(
            tx,
            &call.id,
            &call.name,
            codetether_stream_bounds::bounded_tool_output(&outcome.rendered),
            outcome.success,
            outcome.duration_ms,
        )
        .await;
    }
    let content = host.route(call, &outcome.rendered);
    host.append(codetether_tool_core::messages::tool_result_with_metadata(
        call.id.clone(),
        &call.name,
        outcome.success,
        content,
        outcome.metadata.as_ref(),
    ));
    if let Some(feedback) = outcome.approval_feedback {
        host.append_approval_feedback(&call.id, feedback);
    }
    host.persist().await?;
    host.record_codesearch(outcome.codesearch_miss);
    Ok(())
}
