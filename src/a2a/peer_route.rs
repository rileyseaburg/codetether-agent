//! Reachable routes learned from authenticated LAN discovery.

use dashmap::DashMap;
use std::sync::LazyLock;

static ROUTES: LazyLock<DashMap<String, PeerRoute>> = LazyLock::new(DashMap::new);

/// Callable A2A endpoint learned from LAN discovery.
#[derive(Clone)]
pub(crate) struct PeerRoute {
    /// Normalized A2A base URL.
    pub endpoint: String,
    /// Peer-scoped bearer capability when advertised.
    pub token: Option<String>,
}

/// Inserts or refreshes the callable route for an agent name.
pub(crate) fn register(name: &str, endpoint: &str, token: Option<String>) {
    let token = token.or_else(|| ROUTES.get(name).and_then(|route| route.token.clone()));
    ROUTES.insert(
        name.to_string(),
        PeerRoute {
            endpoint: endpoint.to_string(),
            token,
        },
    );
}

/// Removes a route after its peer expires from discovery.
pub(crate) fn remove(name: &str) {
    ROUTES.remove(name);
}

/// Returns the current route for an agent name.
pub(crate) fn get(name: &str) -> Option<PeerRoute> {
    ROUTES.get(name).map(|route| route.clone())
}

/// Snapshots every currently reachable peer route.
pub(crate) fn list() -> Vec<(String, PeerRoute)> {
    ROUTES
        .iter()
        .map(|route| (route.key().clone(), route.value().clone()))
        .collect()
}
