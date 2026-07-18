//! Result message formatting for sub-agent spawn outcomes.
//!
//! Keeps user-facing spawn strings out of the orchestration module.

use super::spawn_request::SpawnRequest;
use serde_json::json;

/// Appends an optional warning line to a spawn result message.
pub(super) fn with_warning(message: String, warning: Option<&str>) -> String {
    match warning {
        Some(text) => format!("{message}\n{text}"),
        None => message,
    }
}

/// Message for a successful durable spawn.
pub(super) fn success_message(request: &SpawnRequest<'_>, agent_id: &str) -> String {
    format!(
        "Spawned @{} ({agent_id}) on '{}': {}",
        request.name, request.model, request.instructions
    )
}

pub(super) fn detached_message(
    request: &SpawnRequest<'_>,
    agent_id: &str,
    warning: Option<&str>,
) -> String {
    json!({
        "agent_id": agent_id,
        "nickname": request.name,
        "status": "running",
        "warning": warning,
    })
    .to_string()
}

/// Message for a durable spawn whose persistence failed.
pub(super) fn failure_message(request: &SpawnRequest<'_>, error: &anyhow::Error) -> String {
    format!(
        "Failed to spawn @{} durably: child session persistence failed: {error}",
        request.name
    )
}

#[cfg(test)]
#[path = "spawn_messages_tests.rs"]
mod tests;
