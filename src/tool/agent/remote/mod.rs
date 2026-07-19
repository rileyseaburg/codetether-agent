//! First-party agent-tool access to automatically discovered LAN peers.

pub(in crate::tool::agent) mod observation;
mod payload;
mod request;
mod result;
#[cfg(test)]
#[path = "remote_tests.rs"]
mod tests;
mod text;
mod transport;

use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Value, json};

pub(super) async fn message_or_missing(
    name: &str,
    text: &str,
    context_id: Option<&str>,
    parent_session_id: Option<&str>,
    detach: bool,
) -> Result<ToolResult> {
    if context_id.is_some_and(|id| !crate::a2a::session_resolve::usable_as_session_id(id)) {
        return Ok(ToolResult::error(
            "context_id must match [A-Za-z0-9_-] and contain at most 128 bytes",
        ));
    }

    request::execute(name, text, context_id, parent_session_id, detach).await
}

pub(in crate::tool::agent) fn list() -> Vec<Value> {
    crate::a2a::peer_route::list()
        .into_iter()
        .filter(|(name, _)| !super::super::store::contains_name(name, None))
        .map(|(name, route)| {
            json!({
                "name": name,
                "kind": "lan-peer",
                "description": route.description,
                "skills": route.skills,
                "agent_identity_id": route.agent_identity_id,
                "transport": "a2a-mdns"
            })
        })
        .collect()
}
