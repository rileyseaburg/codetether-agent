//! Result message formatting for sub-agent spawn outcomes.
//!
//! Keeps user-facing spawn strings out of the orchestration module.

use super::spawn_request::SpawnRequest;

/// Appends an optional warning line to a spawn result message.
pub(super) fn with_warning(message: String, warning: Option<&str>) -> String {
    match warning {
        Some(text) => format!("{message}\n{text}"),
        None => message,
    }
}

/// Message for a successful durable spawn.
pub(super) fn success_message(request: &SpawnRequest<'_>) -> String {
    format!(
        "Spawned @{} on '{}': {}",
        request.name, request.model, request.instructions
    )
}

/// Message for a successful ephemeral (non-persisted) spawn.
pub(super) fn ephemeral_message(request: &SpawnRequest<'_>) -> String {
    format!(
        "{}\nwarning: ephemeral: true; child session was not persisted",
        success_message(request)
    )
}

/// Message for a durable spawn whose persistence failed.
pub(super) fn failure_message(request: &SpawnRequest<'_>, error: &anyhow::Error) -> String {
    format!(
        "Failed to spawn @{} durably: child session persistence failed: {error}",
        request.name
    )
}
