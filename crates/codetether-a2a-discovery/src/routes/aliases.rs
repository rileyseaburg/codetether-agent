//! Stable identity aliases for ephemeral A2A card routes.

use dashmap::DashMap;
use std::sync::LazyLock;

static ALIASES: LazyLock<DashMap<String, String>> = LazyLock::new(DashMap::new);

pub(super) fn register(identity: Option<&str>, route_name: &str) {
    if let Some(identity) = identity.filter(|identity| *identity != route_name) {
        ALIASES.insert(identity.to_string(), route_name.to_string());
    }
}

pub(super) fn resolve(identity: &str) -> Option<String> {
    ALIASES.get(identity).map(|route| route.clone())
}

pub(super) fn remove_route(route_name: &str) {
    ALIASES.retain(|_, route| route != route_name);
}
