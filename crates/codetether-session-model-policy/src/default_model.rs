//! Seed defaults only when no explicit session model exists.
use codetether_config::Config;
use codetether_session_metadata::SessionMetadata;

/// Adopt `config.default_model` when the session has no model yet.
///
/// Explicit choices always win: a model picked in the TUI, passed on the
/// command line, or restored from a persisted session is left untouched.
/// # Arguments
/// * `metadata` — Session settings to seed.
/// * `config` — Loaded application configuration.
/// # Returns
/// Leaves explicit models unchanged and ignores blank defaults.
/// # Examples
/// ```
/// # fn example<R: codetether_session_model_policy::ModelResolver>(registry: &R) {
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// let config = codetether_config::Config::default();
/// codetether_session_model_policy::seed_default_model(&mut metadata, &config);
/// # }
/// ```
pub fn seed_default_model(metadata: &mut SessionMetadata, config: &Config) {
    if metadata.model.is_some() {
        return;
    }
    let Some(model) = config.default_model.as_deref().map(str::trim) else {
        return;
    };
    if model.is_empty() {
        return;
    }
    tracing::info!(target: "codetether_agent::session::lifecycle::default_model", model, "Seeding session model from configured default");
    metadata.model = Some(model.to_string());
}
