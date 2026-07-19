//! Awaited transport and evidence completion for one remote turn.

use super::super::{observation::types::RemoteTurnGuard, transport};
use crate::a2a::peer_route::PeerRoute;
use crate::tool::ToolResult;
use anyhow::Result;

pub(super) async fn execute(
    name: &str,
    text: &str,
    context_id: Option<&str>,
    route: PeerRoute,
    turn: RemoteTurnGuard,
) -> Result<ToolResult> {
    match transport::send(name, text, context_id, route).await {
        Ok(result) => {
            turn.complete(&result);
            Ok(result)
        }
        Err(error) => {
            turn.fail(&error.to_string());
            Err(error)
        }
    }
}
