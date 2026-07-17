//! Agent-card probing for candidate A2A endpoints.

use crate::a2a::{client::A2AClient, types::AgentCard};
use anyhow::Result;

pub(super) async fn try_fetch_agent_card(endpoint: &str) -> Result<AgentCard> {
    let mut client = A2AClient::new(endpoint);
    if let Ok(token) = std::env::var("CODETETHER_AUTH_TOKEN") {
        client = client.with_token(token);
    }
    client.get_agent_card().await
}
