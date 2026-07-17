//! First-party agent-tool access to automatically discovered LAN peers.

mod payload;
mod result;
mod text;
mod transport;

use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Value, json};

pub(super) async fn message(name: &str, text: &str) -> Option<Result<ToolResult>> {
    if super::super::store::contains(name) {
        return None;
    }
    let route = crate::a2a::peer_route::get(name)?;
    Some(transport::send(name, text, route).await)
}

pub(in crate::tool::agent) fn list() -> Vec<Value> {
    crate::a2a::peer_route::list()
        .into_iter()
        .filter(|(name, _)| !super::super::store::contains(name))
        .map(|(name, route)| {
            json!({
                "name": name,
                "kind": "lan-peer",
                "endpoint": route.endpoint,
                "transport": "a2a-mdns"
            })
        })
        .collect()
}
