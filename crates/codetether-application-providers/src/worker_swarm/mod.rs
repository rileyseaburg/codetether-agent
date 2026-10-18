//! Worker swarm configuration using cached providers and session metadata.
//!
//! [`build`] applies shared task options and updates the session's selected
//! model. [`resolve_model`] retains explicit-model precedence and lazy loading.
use crate::Application;
pub use codetether_a2a_worker_core::swarm_setup::SwarmSetup;
use codetether_session_metadata::SessionMetadata;
mod model;
pub use model::resolve_model;

/// Resolve swarm settings and record the selected model on the session.
/// # Arguments
/// `session` supplies the workspace and receives the model; `metadata` contains
/// task options. `explicit_model` and `model_tier` control provider selection.
/// # Returns
/// The worker's swarm configuration and decomposition strategy.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>(session: &mut codetether_session_metadata::SessionMetadata) {
/// let _setup = codetether_application_providers::worker_swarm::build::<H>(
///     session, &serde_json::Map::new(), Some("provider/model".into()), None).await;
/// # }
/// ```
pub async fn build<H: Application>(
    session: &mut SessionMetadata,
    metadata: &serde_json::Map<String, serde_json::Value>,
    explicit_model: Option<String>,
    model_tier: Option<&str>,
) -> SwarmSetup {
    let setup = codetether_a2a_worker_core::swarm_setup::build_swarm_setup(
        metadata,
        session.directory.as_deref(),
        resolve_model::<H>(explicit_model, model_tier),
    )
    .await;
    if let Some(model) = setup.config.model.clone() {
        session.model = Some(model);
    }
    setup
}
