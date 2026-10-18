//! Best-effort thinker startup and initialization diagnostics.
use super::{Client, Host};
use crate::ThinkerConfig;
use std::sync::Arc;
/// Build a thinker client, returning `None` when disabled or unavailable.
///
/// A failure here is not fatal: the cognition loop falls back to deterministic
/// phase text.
pub fn initialize<H: Host>(config: Option<ThinkerConfig>) -> Option<Arc<Client<H>>> {
    let config = config?;
    if !config.enabled {
        return None;
    }
    match Client::<H>::new(config) {
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
