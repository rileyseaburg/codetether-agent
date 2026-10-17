//! Native startup specialization for shared first-turn coordination.
use crate::tool::ToolResult;
#[path = "spawn_run/start.rs"]
pub(in crate::tool::agent) mod start;
pub(super) async fn kick_off(agent_id: &str, detach: bool) -> anyhow::Result<ToolResult> {
    codetether_child_turn::kick_off::<super::turn_host::ApplicationHost>(agent_id, detach).await
}
