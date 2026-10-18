//! Sequential sub-agent tool batches and transcript result construction.
//!
//! [execute] checks readiness before each call, appends a tool-role result with
//! image attachments, and stops after recording a timed-out call. [Host] binds
//! native control, single-tool execution, and transcript insertion.
mod completed;
mod host;
use super::ToolCall;
use codetether_provider_core::{ContentPart, Message, Role};
pub use completed::Completed;
pub use host::Host;

/// Execute an ordered batch of tool calls and record each result.
/// # Arguments
/// * `host` — Readiness, single-call execution, and transcript bindings.
/// * `calls` — Tool calls in provider response order.
/// # Returns
/// Whether a completed call timed out and terminated the remaining batch.
/// # Errors
/// Propagates readiness failure before the affected tool call starts.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_swarm_core::agent_loop::tools::Host) -> anyhow::Result<()> {
/// assert!(!codetether_swarm_core::agent_loop::tools::execute(host, vec![]).await?);
/// # Ok(()) }
/// ```
pub async fn execute(host: &mut impl Host, calls: Vec<ToolCall>) -> anyhow::Result<bool> {
    for call in calls {
        host.ready().await?;
        let completed = host.execute(call).await;
        let mut content = vec![ContentPart::ToolResult {
            tool_call_id: completed.id,
            content: completed.output,
        }];
        content.extend(completed.images);
        host.record(Message {
            role: Role::Tool,
            content,
        });
        tracing::debug!(target: "codetether_agent::swarm::executor::agent_loop::tools", tool = %completed.name, "Recorded tool result");
        if completed.timed_out {
            return Ok(true);
        }
    }
    Ok(false)
}
