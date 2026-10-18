//! Native controller binding for shared revision-bound goal edits.
use crate::tool::ToolResult;
use serde_json::Value;
pub(super) async fn run(input: Value) -> anyhow::Result<ToolResult> {
    codetether_goal_tools::edit::<super::host::Native>(input).await
}
