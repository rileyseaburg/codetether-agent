//! Transparent durable child loading before collaboration operations.

use crate::{EnsureOpen, MessagingHost, RuntimeContext};
use codetether_tool_core::ToolResult;

use anyhow::Result;

pub(crate) async fn ready<H: MessagingHost>(
    context: &RuntimeContext,
    target: &str,
) -> Result<Option<ToolResult>> {
    let owner = context.session_id.as_deref();
    match H::open(target, owner, context).await? {
        EnsureOpen::Ready {
            agent_id,
            name,
            resumed,
        } => {
            if resumed {
                tracing::info!(target:"codetether_agent::tool::collaboration::ensure",%agent_id, agent = %name, "Reloaded durable child for collaboration");
            }
            Ok(None)
        }
        EnsureOpen::Missing => Ok(Some(ToolResult::error(format!("Agent {target} not found")))),
        EnsureOpen::Rejected(result) => Ok(Some(result)),
    }
}
