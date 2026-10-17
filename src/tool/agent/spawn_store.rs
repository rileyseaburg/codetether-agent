//! Native storage specialization for shared durable child registration.
use super::spawn_request::SpawnRequest;
use crate::session::Session;
#[path = "spawn_store/host.rs"]
mod host;
pub(super) async fn persist_spawned_agent(
    request: &SpawnRequest<'_>,
    session: Session,
) -> anyhow::Result<String> {
    codetether_agent_spawn::persist_spawned_agent::<host::ApplicationStore>(request, session).await
}
