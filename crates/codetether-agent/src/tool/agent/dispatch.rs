//! Native specialization retained for legacy dispatch regression tests.
use super::params::Params;
use crate::tool::ToolResult;
use anyhow::Result;
pub(super) async fn execute(params: &Params) -> Result<ToolResult> {
    codetether_agent_tool::dispatch::<super::tool_impl::ApplicationHost>(params).await
}
#[cfg(test)]
#[path = "dispatch_tests.rs"]
mod tests;
