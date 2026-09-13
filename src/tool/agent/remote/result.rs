//! Model-facing rendering of a [`PeerReply`].
//!
//! The model gets a JSON envelope naming the peer and transport; the TUI
//! transcript records the plain reply separately (see `request/run.rs`).

use super::reply::PeerReply;
use crate::tool::ToolResult;
use serde_json::json;

pub(in crate::tool::agent) fn render(name: &str, reply: &PeerReply) -> ToolResult {
    let output = json!({
        "agent": name,
        "response": reply.text,
        "transport": "a2a-mdns"
    });
    let rendered = serde_json::to_string_pretty(&output).unwrap_or_else(|_| reply.text.clone());
    match reply.failed {
        true => ToolResult::error(rendered),
        false => ToolResult::success(rendered),
    }
}
