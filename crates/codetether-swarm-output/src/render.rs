//! Context-safe rendering of a sub-agent tool result.
use crate::{Context, agent_loop_large_result, large_result};
use codetether_swarm_core::context::truncate_single_result;
use codetether_tool_core::feedback::render_with_dir;
use std::sync::Arc;

/// Bound tool output and apply the shared feedback envelope.
///
/// # Arguments
/// * `context` — Borrowed provider, model, deadline, and working directory.
/// * `name` — Tool name shown in feedback and summarization prompts.
/// * `output` — Original tool output text.
/// * `success` — Tool execution outcome retained in feedback.
/// # Returns
/// Rendered output and whether large-output processing timed out.
/// # Examples
/// ```no_run
/// # async fn example(context: codetether_swarm_output::Context<'_>) {
/// let (_, timed_out) = codetether_swarm_output::render(context, "read", "small output", true).await;
/// assert!(!timed_out);
/// # }
/// ```
pub async fn render(
    context: Context<'_>,
    name: &str,
    output: &str,
    success: bool,
) -> (String, bool) {
    let (output, timed_out) = if output.len() > large_result::RLM_THRESHOLD_CHARS {
        agent_loop_large_result::process(
            output,
            name,
            Arc::clone(context.provider),
            context.model,
            context.deadline,
        )
        .await
    } else {
        (
            truncate_single_result(output, large_result::SIMPLE_TRUNCATE_CHARS),
            false,
        )
    };
    (
        render_with_dir(name, success, context.working_dir, &output),
        timed_out,
    )
}
