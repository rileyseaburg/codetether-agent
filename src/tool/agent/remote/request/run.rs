//! Awaited transport and evidence completion for one remote turn.

use super::super::{observation::types::RemoteTurnGuard, reply::PeerReply, result, transport};
use crate::a2a::peer_route::PeerRoute;
use crate::tool::ToolResult;
use anyhow::Result;

/// Send one turn to a peer, record what it said, and render it for the model.
pub(super) async fn execute(
    name: &str,
    text: &str,
    context_id: Option<&str>,
    route: PeerRoute,
    turn: RemoteTurnGuard,
) -> Result<ToolResult> {
    let owner = turn.owner_session_id.as_deref();
    match transport::send(name, text, context_id, owner, route).await {
        Ok(reply) => {
            turn.settle(&reply);
            Ok(result::render(name, &reply))
        }
        Err(error) => {
            turn.settle(&PeerReply::transport_failure(&error.to_string()));
            Err(error)
        }
    }
}
