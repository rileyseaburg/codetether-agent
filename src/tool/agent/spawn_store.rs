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
) -> Result<()> {
    save_session(request.name, &session).await?;
    store::insert(
        request.name.to_string(),
        AgentEntry {
            instructions: request.instructions.to_string(),
            session,
            parent: None,
            owner_session_id: request.parent_session_id.map(str::to_string),
            depth: 0,
            model_id: Some(request.model.to_string()),
        },
    );
    Ok(())
}

async fn save_session(name: &str, session: &Session) -> Result<()> {
    session.save().await.map_err(|error| {
        tracing::warn!(agent = %name, error = %error, "Failed to save spawned agent session");
        error
    })
}
