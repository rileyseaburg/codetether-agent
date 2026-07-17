//! Structured reporting and health updates for interrupted WebSockets.

use super::super::TransportHealth;

pub(super) fn record(health: &TransportHealth, error: Option<&anyhow::Error>) {
    tracing::warn!(
        error = error.map(ToString::to_string),
        "Codex transport retrying privately"
    );
    health.mark_interrupted();
}
