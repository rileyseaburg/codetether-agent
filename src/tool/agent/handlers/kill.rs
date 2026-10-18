//! Native owner-scoped specialization for shared permanent child removal.
use crate::tool::ToolResult;
pub(in crate::tool::agent) async fn handle(
    target: &str,
    parent: Option<&str>,
) -> anyhow::Result<ToolResult> {
    codetether_agent_lifecycle::kill::<super::host::ApplicationHost>(target, parent).await
}
