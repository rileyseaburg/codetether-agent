//! Native runtime binding for shared goal creation.
use super::{create::Args, host::Native};
use crate::tool::ToolResult;
pub(super) async fn run(args: Args) -> anyhow::Result<ToolResult> {
    codetether_goal_tools::create::<Native>(args).await
}
