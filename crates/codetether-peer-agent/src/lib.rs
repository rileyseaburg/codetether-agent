//! First-party agent-tool access to automatically discovered LAN peers.
//!
//! [message_or_missing] validates context, resolves a peer, and records its turn.
//! [list] combines local identity with discovered peers while [LocalNames]
//! retains local-child precedence. [observation] exposes owner-scoped evidence.

mod list;
pub mod observation;
mod payload;
mod poll;
pub mod reply;
mod request;
pub mod result;
#[cfg(test)]
#[path = "remote_tests.rs"]
mod tests;
mod text;
mod transport;

use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;

/// Send a validated turn to a discovered peer or return the existing missing result.
/// # Arguments
/// * `name` — Discovered peer name or supported identity alias.
/// * `text` — User request text.
/// * `context_id` — Optional reusable remote conversation ID.
/// * `parent_session_id` — Optional owner of observed turn evidence.
/// * `detach` — Whether to return before the remote turn settles.
/// # Returns
/// Existing peer result, detached response, or validation/missing-peer failure.
/// # Errors
/// Propagates synchronous transport and polling failures.
/// # Examples
/// See [observation] for reading evidence after a turn.
pub async fn message_or_missing(
    name: &str,
    text: &str,
    context_id: Option<&str>,
    parent_session_id: Option<&str>,
    detach: bool,
) -> Result<ToolResult> {
    if context_id.is_some_and(|id| !codetether_a2a_http::conversation::usable_as_session_id(id)) {
        return Ok(ToolResult::error(
            "context_id must match [A-Za-z0-9_-] and contain at most 128 bytes",
        ));
    }

    request::execute(name, text, context_id, parent_session_id, detach).await
}

/// List local identity and discovered peers, retaining local child-name precedence.
/// # Arguments
/// This function takes no value arguments; [LocalNames] supplies local collisions.
/// # Returns
/// The existing JSON peer-list rows.
/// # Examples
/// Implement [LocalNames] using the application's live child registry.
pub fn list<H: LocalNames>() -> Vec<Value> {
    list::all::<H>()
}

/// Local child names that must shadow discovered peers in a combined listing.
/// # Implementors
/// Applications query their existing live child registry here.
pub trait LocalNames {
    /// Return whether any local owner scope contains this display name.
    fn contains(name: &str) -> bool;
}
