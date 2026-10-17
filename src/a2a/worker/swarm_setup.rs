//! Session adapter for shared worker swarm configuration.
use super::resolve_swarm_model;
pub(super) use codetether_a2a_worker_core::swarm_setup::SwarmSetup;
pub(super) async fn build_swarm_setup(
    session: &mut crate::session::Session,
    metadata: &serde_json::Map<String, serde_json::Value>,
    explicit_model: Option<String>,
    model_tier: Option<&str>,
) -> SwarmSetup {
    let setup = codetether_a2a_worker_core::swarm_setup::build_swarm_setup(
        metadata,
        session.metadata.directory.as_deref(),
        resolve_swarm_model(explicit_model, model_tier),
    )
    .await;
    if let Some(model) = setup.config.model.clone() {
        session.metadata.model = Some(model);
    }
    setup
}
