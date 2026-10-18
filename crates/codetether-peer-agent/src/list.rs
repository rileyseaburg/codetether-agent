//! Unified rendering of this process and discovered LAN peers.

use serde_json::{Value, json};

pub(super) fn all<H: crate::LocalNames>() -> Vec<Value> {
    let name = codetether_a2a_discovery::local_identity::current_name();
    let mut agents = local(name.clone()).into_iter().collect::<Vec<_>>();
    agents.extend(
        codetether_a2a_discovery::routes::list()
            .into_iter()
            .filter(|(peer, _)| name.as_deref() != Some(peer))
            .filter(|(name, _)| !H::contains(name))
            .map(|(name, route)| {
                json!({
                    "name": name, "kind": "lan-peer", "self": false,
                    "description": route.description, "skills": route.skills,
                    "agent_identity_id": route.agent_identity_id,
                    "transport": "a2a-mdns"
                })
            }),
    );
    agents
}

fn local(name: Option<String>) -> Option<Value> {
    name.map(|name| {
        json!({
            "name": name,
            "kind": "lan-peer",
            "self": true,
            "description": "This CodeTether process",
            "transport": "a2a-mdns"
        })
    })
}

#[cfg(test)]
#[path = "list_tests.rs"]
mod tests;
