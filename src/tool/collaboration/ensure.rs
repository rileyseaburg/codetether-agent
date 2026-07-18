//! Transparent durable child loading before collaboration operations.

use super::context::RuntimeContext;
use crate::tool::ToolResult;
use crate::tool::agent::residency::EnsureOpen;
use anyhow::Result;

pub(super) async fn ready(
    context: &RuntimeContext,
    target: &str,
) -> Result<Option<ToolResult>> {
    let owner = context.session_id.as_deref();
    match crate::tool::agent::residency::open(target, owner, context.resume_config()).await? {
        EnsureOpen::Ready { agent_id, name, resumed } => {
            if resumed {
                tracing::info!(%agent_id, agent = %name, "Reloaded durable child for collaboration");
            }
            Ok(None)
        }
        EnsureOpen::Missing => Ok(Some(ToolResult::error(format!(
            "Agent {target} not found"
        )))),
        EnsureOpen::Rejected(result) => Ok(Some(result)),
    }
}
