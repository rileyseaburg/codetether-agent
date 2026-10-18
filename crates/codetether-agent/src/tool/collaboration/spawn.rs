//! Application binding for the shared durable-child spawn tool.
pub(super) type SpawnAgentTool =
    codetether_collaboration_tools::SpawnAgentTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const SpawnAgentTool: SpawnAgentTool = SpawnAgentTool::new();
#[cfg(test)]
#[path = "spawn_tests.rs"]
mod tests;
