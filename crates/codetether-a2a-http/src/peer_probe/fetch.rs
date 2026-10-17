//! Agent-card probing for candidate A2A endpoints.

use crate::A2AClient;
use anyhow::Result;
use codetether_a2a_core::AgentCard;

/// Fetch a candidate endpoint's agent card using configured host authentication.
///
/// # Arguments
/// * `endpoint` - Base URL to probe.
/// # Returns
/// The remote agent card.
/// # Errors
/// Returns an error if the HTTP request or card decoding fails.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_a2a_http::peer_probe::try_fetch_agent_card;
/// let card = try_fetch_agent_card("http://localhost:4096").await?;
/// assert!(!card.url.is_empty());
/// # Ok(()) }
/// ```
/// Reads `CODETETHER_AUTH_TOKEN` for each call; absent or non-Unicode values
/// leave the client unauthenticated.
pub async fn try_fetch_agent_card(endpoint: &str) -> Result<AgentCard> {
    let mut client = A2AClient::new(endpoint);
    if let Ok(token) = std::env::var("CODETETHER_AUTH_TOKEN") {
        client = client.with_token(token);
    }
    client.get_agent_card().await
}
