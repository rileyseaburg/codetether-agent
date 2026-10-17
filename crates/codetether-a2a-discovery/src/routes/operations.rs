//! Registry updates, identity lookup and snapshots.
use super::*;

/// Inserts or refreshes the callable route for an agent name.
pub fn register(card: &AgentCard, endpoint: &str, token: Option<String>) {
    let identity = codetether_a2a_core::identity_extension::from_card(card);
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
pub fn remove(name: &str) {
    ROUTES.remove(name);
    aliases::remove_route(name);
}

/// Returns the current route for an agent name.
pub fn get(name: &str) -> Option<PeerRoute> {
    direct(name).or_else(|| aliases::resolve(name).and_then(|route| direct(&route)))
}

fn direct(name: &str) -> Option<PeerRoute> {
    ROUTES.get(name).map(|route| route.clone())
}

/// Snapshots every currently reachable peer route.
pub fn list() -> Vec<(String, PeerRoute)> {
    ROUTES
        .iter()
        .map(|route| (route.key().clone(), route.value().clone()))
        .collect()
}
