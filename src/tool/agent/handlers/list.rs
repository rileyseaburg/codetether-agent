//! Native owner-scoped specialization of the shared agent roster.
use crate::tool::ToolResult;
pub(in crate::tool::agent) async fn handle(parent: Option<&str>) -> anyhow::Result<ToolResult> {
    codetether_agent_view::legacy_list::<crate::tool::agent::bridge::ApplicationHost>(parent)
}
