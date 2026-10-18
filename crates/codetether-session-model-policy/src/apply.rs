//! Ordered application of configured session model policy.
use crate::{ModelResolver, apply_subcall_model, seed_default_model};
use codetether_config::Config;
use codetether_session_metadata::SessionMetadata;
/// Apply configured RLM settings, seed the default model and reset subcall resolution.
/// # Arguments
/// * `metadata` — Session settings to update.
/// * `config` — Loaded application configuration.
/// * `registry` — Optional model resolver.
/// # Returns
/// Updates metadata in the same order as native configuration application.
/// # Examples
/// ```
/// # fn example<R: codetether_session_model_policy::ModelResolver>(registry: &R) {
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// let config = codetether_config::Config::default();
/// codetether_session_model_policy::apply_config(&mut metadata, &config, Some(registry));
/// # }
/// ```
pub fn apply_config<R: ModelResolver>(
    metadata: &mut SessionMetadata,
    config: &Config,
    registry: Option<&R>,
) {
    metadata.rlm = config.rlm.clone();
    seed_default_model(metadata, config);
    apply_subcall_model(metadata, registry);
}
