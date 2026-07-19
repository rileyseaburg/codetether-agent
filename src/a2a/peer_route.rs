//! Reachable routes learned from authenticated LAN discovery.

use dashmap::DashMap;
use std::sync::LazyLock;

use crate::a2a::types::AgentCard;

#[path = "peer_route_alias.rs"]
mod aliases;

static ROUTES: LazyLock<DashMap<String, PeerRoute>> = LazyLock::new(DashMap::new);

/// Callable A2A endpoint learned from LAN discovery.
#[derive(Clone)]
pub(crate) struct PeerRoute {
    /// Normalized A2A base URL.
    pub endpoint: String,
    /// Peer-scoped bearer capability when advertised.
    pub token: Option<String>,
    /// Human-readable responsibility advertised by the peer.
    pub description: String,
    /// Stable skill identifiers advertised by the peer.
    pub skills: Vec<String>,
    /// Stable provenance identity advertised by the peer.
    pub agent_identity_id: Option<String>,
}

/// Inserts or refreshes the callable route for an agent name.
pub(crate) fn register(card: &AgentCard, endpoint: &str, token: Option<String>) {
    let identity = crate::a2a::agent_identity::from_card(card);
    let token = token.or_else(|| ROUTES.get(&card.name).and_then(|route| route.token.clone()));
    aliases::remove_route(&card.name);
    ROUTES.insert(
        card.name.clone(),
        PeerRoute {
            endpoint: endpoint.to_string(),
            token,
            description: card.description.clone(),
            skills: card.skills.iter().map(|skill| skill.id.clone()).collect(),
            agent_identity_id: identity.clone(),
        },
    );
    aliases::register(identity.as_deref(), &card.name);
}

/// Removes a route after its peer expires from discovery.
pub(crate) fn remove(name: &str) {
    ROUTES.remove(name);
    aliases::remove_route(name);
}

/// Returns the current route for an agent name.
pub(crate) fn get(name: &str) -> Option<PeerRoute> {
    direct(name).or_else(|| aliases::resolve(name).and_then(|route| direct(&route)))
}

fn direct(name: &str) -> Option<PeerRoute> {
    ROUTES.get(name).map(|route| route.clone())
}

/// Snapshots every currently reachable peer route.
pub(crate) fn list() -> Vec<(String, PeerRoute)> {
    ROUTES
        .iter()
        .map(|route| (route.key().clone(), route.value().clone()))
        .collect()
}

#[cfg(test)]
#[path = "peer_route_tests.rs"]
mod tests;
