//! Persistence helpers for spawned agents.
//!
//! Saves newly created sessions and registers them in the in-memory agent
//! store only after durable persistence succeeds.

use super::spawn_request::SpawnRequest;
use super::store::{self, AgentEntry};
use crate::session::Session;
use anyhow::Result;

/// Saves a spawned agent session and registers it in the in-memory store.
pub(super) async fn persist_spawned_agent(
    request: &SpawnRequest<'_>,
    session: Session,
) -> Result<String> {
    save_session(request.name, &session).await?;
    let (parent, depth) = store::lineage_for_session(request.parent_session_id);
    let entry = AgentEntry {
        name: request.name.to_string(),
        instructions: request.instructions.to_string(),
        session,
        parent,
        owner_session_id: request.parent_session_id.map(str::to_string),
        depth,
        model_id: Some(request.model.to_string()),
    };
    super::persistence::save(request.name, &entry).await?;
    let agent_id = entry.session.id.clone();
    store::insert_reserved(entry);
    super::collaboration_runtime::thread_status::initialize(&agent_id);
    Ok(agent_id)
}

async fn save_session(name: &str, session: &Session) -> Result<()> {
    session.save().await.map_err(|error| {
        tracing::warn!(agent = %name, error = %error, "Failed to save spawned agent session");
        error
    })
}
