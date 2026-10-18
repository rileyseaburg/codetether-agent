//! Native specialization for shared active-turn interruption.
use crate::tool::ToolResult;
pub(in crate::tool::agent) async fn handle(
    target: &str,
    parent: Option<&str>,
    marker_enabled: bool,
) -> anyhow::Result<ToolResult> {
    codetether_agent_lifecycle::interrupt::<super::host::ApplicationHost>(
        target,
        parent,
        marker_enabled,
    )
    .await
}
#[cfg(test)]
#[path = "interrupt/idle_tests.rs"]
mod idle_tests;
#[cfg(test)]
use codetether_agent_lifecycle as marker;
#[cfg(test)]
#[path = "interrupt/test_support.rs"]
mod test_support;
#[cfg(test)]
#[path = "interrupt/tests.rs"]
mod tests;
