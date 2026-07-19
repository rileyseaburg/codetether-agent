//! CodeTether agent-identity extension for A2A agent cards.

use crate::a2a::types::{AgentCard, AgentExtension};
use serde_json::json;

const URI: &str = "https://codetether.run/extensions/agent-identity/v1";

/// Adds the routable provenance identity extension to an agent card.
pub(crate) fn attach(card: &mut AgentCard, identity: &str) {
    card.capabilities.extensions.push(AgentExtension {
        uri: URI.to_string(),
        description: Some("Routable CodeTether provenance identity".to_string()),
        required: false,
        params: Some(json!({ "agentIdentityId": identity })),
    });
}

/// Extracts a non-empty routable provenance identity from an agent card.
pub(crate) fn from_card(card: &AgentCard) -> Option<String> {
    card.capabilities
        .extensions
        .iter()
        .find(|extension| extension.uri == URI)
        .and_then(|extension| extension.params.as_ref())
        .and_then(|params| params["agentIdentityId"].as_str())
        .map(str::trim)
        .filter(|identity| !identity.is_empty())
        .map(ToString::to_string)
}

#[cfg(test)]
#[path = "agent_identity_tests.rs"]
mod tests;
