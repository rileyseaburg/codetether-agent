//! Thinker client initialization from configuration.

use std::sync::Arc;

use super::{ThinkerClient, ThinkerConfig};

/// Build a thinker client, returning `None` when disabled or unavailable.
///
/// A failure here is not fatal: the cognition loop falls back to deterministic
/// phase text.
pub(super) fn init_thinker(config: Option<ThinkerConfig>) -> Option<Arc<ThinkerClient>> {
    let config = config?;
    if !config.enabled {
        return None;
    }
    match ThinkerClient::new(config) {
        Ok(client) => {
            tracing::info!(
                backend = ?client.config().backend,
                endpoint = %client.config().endpoint,
                model = %client.config().model,
                "Cognition thinker initialized"
            );
            Some(Arc::new(client))
        }
        Err(error) => {
            tracing::warn!(%error, "Failed to initialize cognition thinker; using default thoughts");
            None
        }
    }
}
