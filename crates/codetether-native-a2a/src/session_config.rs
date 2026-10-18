//! Apply A2A-specific runtime configuration to sessions.

use crate::Session;

/// Apply the configured default model to an A2A session.
pub async fn configure<S: Session>(session: &mut S) {
    codetether_a2a_http::conversation::lifecycle::configure(
        session.model_mut(),
        codetether_config::a2a_runtime::configured_model(),
    )
    .await;
}

/// Best-effort persistence for A2A conversation sessions.
pub async fn persist<S: Session>(session: &S) {
    codetether_a2a_http::conversation::lifecycle::persist(session.id(), session.save()).await;
}
