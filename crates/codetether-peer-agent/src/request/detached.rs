//! Background dispatch for interactive remote-agent messages.

use super::run;
use crate::observation::types::RemoteTurnGuard;
use codetether_a2a_discovery::routes::PeerRoute;
use codetether_tool_core::ToolResult;

pub(super) fn spawn(
    name: &str,
    text: &str,
    context_id: Option<&str>,
    route: PeerRoute,
    turn: RemoteTurnGuard,
) -> ToolResult {
    let owned_name = name.to_string();
    let owned_text = text.to_string();
    let owned_context = context_id.map(ToString::to_string);
    let owner = turn.owner_session_id.clone();
    tokio::spawn(async move {
        let outcome = run::execute(
            &owned_name,
            &owned_text,
            owned_context.as_deref(),
            route,
            turn,
        )
        .await;
        super::completion::publish(&owned_name, owner.as_deref(), outcome);
    });
    ToolResult::success(format!(
        "Message dispatched to @{name}; its reply will return to this conversation. Open /agents to inspect its live transcript"
    ))
}
