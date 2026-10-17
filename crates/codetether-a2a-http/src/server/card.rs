//! Public agent-card discovery response.
use super::*;

/// Get agent card handler
pub(super) async fn get_agent_card<H: ServerHost>(State(server): State<H>) -> Json<AgentCard> {
    Json(server.agent_card().clone())
}
