//! Apply A2A-specific runtime configuration to sessions.

use crate::session::Session;

/// Apply the configured default model to an A2A session.
pub async fn configure(session: &mut Session) {
    if let Some(model) = configured_model().await {
        session.metadata.model = Some(model);
    }
}

async fn configured_model() -> Option<String> {
    match model_from_env() {
        Some(model) => Some(model),
        None => async_load_config_model().await,
    }
}

fn model_from_env() -> Option<String> {
    std::env::var("CODETETHER_DEFAULT_MODEL")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

async fn async_load_config_model() -> Option<String> {
    match crate::config::Config::load().await {
        Ok(config) => config
            .default_model
            .filter(|value| !value.trim().is_empty()),
        Err(e) => {
            tracing::debug!(error = %e, "Failed to load config for A2A session model");
            None
        }
    }
}

/// Best-effort persistence for A2A conversation sessions.
pub async fn persist(session: &Session) {
    if let Err(e) = session.save().await {
        tracing::debug!(session_id = %session.id, error = %e, "Failed to persist A2A session");
    }
}
