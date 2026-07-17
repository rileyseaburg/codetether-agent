//! Authenticated A2A transport for one discovered peer turn.

use super::{payload, result};
use crate::a2a::{client::A2AClient, peer_route::PeerRoute};
use crate::tool::ToolResult;
use anyhow::{Context, Result};

pub(super) async fn send(name: &str, text: &str, route: PeerRoute) -> Result<ToolResult> {
    let mut client = A2AClient::new(&route.endpoint);
    let token = route
        .token
        .or_else(|| std::env::var("CODETETHER_AUTH_TOKEN").ok());
    if let Some(token) = token {
        client = client.with_token(token);
    }
    tracing::info!(peer_name = %name, endpoint = %route.endpoint, "Delegating to LAN peer");
    let response = client
        .send_message(payload::build(text))
        .await
        .with_context(|| format!("LAN peer {name} call failed at {}", route.endpoint))?;
    tracing::info!(peer_name = %name, endpoint = %route.endpoint, "LAN peer replied");
    Ok(result::render(name, response))
}
