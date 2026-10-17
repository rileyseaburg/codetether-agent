//! Apply A2A-specific runtime configuration to sessions.

use crate::session::Session;

/// Apply the configured default model to an A2A session.
pub async fn configure(session: &mut Session) {
    if let Some(model) = codetether_config::a2a_runtime::configured_model().await {
        session.metadata.model = Some(model);
    }
}

/// Best-effort persistence for A2A conversation sessions.
pub async fn persist(session: &Session) {
    if let Err(e) = session.save().await {
        tracing::debug!(session_id = %session.id, error = %e, "Failed to persist A2A session");
    }
}
