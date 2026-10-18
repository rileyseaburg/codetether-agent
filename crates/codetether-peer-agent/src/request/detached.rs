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
    tokio::spawn(async move {
        if let Err(error) = run::execute(
            &owned_name,
            &owned_text,
            owned_context.as_deref(),
            route,
            turn,
        )
        .await
        {
            tracing::warn!(target: "codetether_agent::tool::agent::message::remote::request::detached", peer_name = %owned_name, %error, "Detached LAN peer turn failed");
        }
    });
    ToolResult::success(format!(
        "Message dispatched to @{name}; open /agents to inspect its live transcript"
    ))
}
