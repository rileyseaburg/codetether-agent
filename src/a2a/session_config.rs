//! Apply A2A-specific runtime configuration to sessions.

use crate::session::Session;

/// Apply the configured default model to an A2A session.
pub async fn configure(session: &mut Session) {
    codetether_a2a_http::conversation::lifecycle::configure(
        &mut session.metadata.model,
        codetether_config::a2a_runtime::configured_model(),
    )
    .await;
}

/// Best-effort persistence for A2A conversation sessions.
pub async fn persist(session: &Session) {
    codetether_a2a_http::conversation::lifecycle::persist(&session.id, session.save()).await;
}
