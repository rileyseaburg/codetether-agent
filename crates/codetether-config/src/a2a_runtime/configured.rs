//! Configuration fallback for A2A session models.

pub(super) async fn async_load_config_model() -> Option<String> {
    match crate::Config::load().await {
        Ok(config) => config
            .default_model
            .filter(|value| !value.trim().is_empty()),
        Err(e) => {
            tracing::debug!(target: "codetether_agent::a2a::session_config",error = %e, "Failed to load config for A2A session model");
            None
        }
    }
}
