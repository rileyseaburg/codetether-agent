//! Reset and resolve optional RLM subcall model bindings.
use crate::ModelResolver;
use codetether_session_metadata::SessionMetadata;

/// Reset and, when a registry is available, resolve the subcall model
/// selector carried by `metadata.rlm`.
/// # Arguments
/// * `metadata` — Session settings whose cached resolver state is reset.
/// * `registry` — Optional model resolver.
/// # Returns
/// Clears both cached subcall fields before resolving when possible.
/// # Examples
/// ```
/// # fn example<R: codetether_session_model_policy::ModelResolver>(registry: &R) {
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// let config = codetether_config::Config::default();
/// codetether_session_model_policy::apply_subcall_model(&mut metadata, Some(registry));
/// # }
/// ```
pub fn apply_subcall_model<R: ModelResolver>(metadata: &mut SessionMetadata, registry: Option<&R>) {
    metadata.subcall_provider = None;
    metadata.subcall_model_name = None;
    if let Some(registry) = registry {
        resolve_subcall_provider(metadata, registry);
    }
}
/// Resolve an unset subcall provider, logging lookup failures.
/// # Arguments
/// * `metadata` — Session settings containing the selector and optional cache.
/// * `registry` — Model resolver for the configured selector.
/// # Returns
/// Keeps cached providers and logs lookup failure without returning an error.
/// # Examples
/// ```
/// # fn example<R: codetether_session_model_policy::ModelResolver>(registry: &R) {
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// let config = codetether_config::Config::default();
/// codetether_session_model_policy::resolve_subcall_provider(&mut metadata, registry);
/// # }
/// ```
pub fn resolve_subcall_provider<R: ModelResolver>(metadata: &mut SessionMetadata, registry: &R) {
    if metadata.subcall_provider.is_some() {
        return;
    }
    let Some(selector) = metadata.rlm.subcall_model.as_deref() else {
        return;
    };
    match registry.resolve_model(selector) {
        Ok((provider, model_name)) => {
            tracing::debug!(target: "codetether_agent::session::lifecycle::subcall_model", subcall_model = %model_name, "RLM: resolved subcall provider");
            metadata.subcall_provider = Some(provider);
            metadata.subcall_model_name = Some(model_name);
        }
        Err(error) => tracing::warn!(target: "codetether_agent::session::lifecycle::subcall_model",
            configured = selector,
            %error,
            "RLM subcall_model resolution failed; subcalls will use root model"
        ),
    }
}
