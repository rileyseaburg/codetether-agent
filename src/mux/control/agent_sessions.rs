//! Mux sessions projected into the first-party agent roster.

use anyhow::Result;
use serde_json::{Value, json};

/// Project authenticated mux sessions into agent-tool roster entries.
///
/// # Errors
///
/// Returns an error when registered mux sessions cannot be listed.
pub(crate) async fn agent_sessions() -> Result<Vec<Value>> {
    Ok(super::list_sessions()
        .await?
        .into_iter()
        .map(|session| {
            let active = session
                .windows
                .iter()
                .find(|item| item.id == session.active_window);
            let runtime = session.runtime.as_ref();
            let principal = runtime.map(|item| &item.principal);
            let display_name = principal
                .map(|item| item.agent_name.as_str())
                .filter(|name| !name.is_empty())
                .unwrap_or(&session.name);
            json!({
                "agent_id": &session.name,
                "name": display_name,
                "kind": "mux_session",
                "transport": "mux",
                "reachable": session.reachable,
                "workspace": active.map(|item| &item.workspace),
                "window": active.map(|item| item.id),
                "session_title": runtime.map(|item| &item.session_title),
                "status": runtime.map(|item| if item.processing { "working" } else { "idle" }),
                "current_tool": runtime.and_then(|item| item.current_tool.as_deref()),
                "needs_interaction": runtime.is_some_and(|item| item.needs_interaction),
                "lagging": runtime.is_some_and(|item| item.lagging),
                "agent_identity_id": principal.and_then(|item| item.agent_identity_id.as_deref()),
                "persona_id": principal.and_then(|item| item.persona_id.as_deref()),
                "spiffe_id": principal.and_then(|item| item.spiffe_id.as_deref()),
                "provenance_id": principal.and_then(|item| item.provenance_id.as_deref()),
            })
        })
        .collect())
}

pub(crate) fn is_agent_route(item: &Value, name: &str) -> bool {
    item["agent_id"] == name
}

#[cfg(test)]
#[path = "agent_sessions_tests.rs"]
mod tests;
