//! Persistence helpers for spawned agents.
//!
//! Saves newly created sessions and registers them in the in-memory agent
//! store only after durable persistence succeeds.

use super::store::{self, AgentEntry};
use crate::session::Session;
use anyhow::Result;

/// Saves a spawned agent session and registers it in the in-memory store.
pub(super) async fn persist_spawned_agent(
    name: &str,
    instructions: &str,
    session: Session,
) -> Result<()> {
    save_session(name, &session).await?;
    store::insert(
        name.to_string(),
        AgentEntry {
            instructions: instructions.to_string(),
            session,
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
