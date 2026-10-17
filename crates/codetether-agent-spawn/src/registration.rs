//! Persistence helpers for spawned agents.
//!
//! Saves newly created sessions and registers them in the in-memory agent
//! store only after durable persistence succeeds.

use crate::Registration;
use anyhow::Result;
use codetether_agent_input::SpawnRequest;
use codetether_agent_registry::AgentEntry;
use codetether_collaboration_state::thread_status;

/// Save a spawned session and manifest before live registration and status setup.
/// # Arguments
/// * `request` — Validated spawn settings and parent identity.
/// * `session` — Newly constructed child session to persist and register.
/// # Returns
/// The durable child session ID after successful registration.
/// # Errors
/// Propagates session and manifest save failures without live insertion.
/// # Examples
/// Use an application [Registration] implementation to provide native storage.
pub async fn persist_spawned_agent<H: Registration>(
    request: &SpawnRequest<'_>,
    session: H::Session,
) -> Result<String> {
    save_session::<H>(request.name, &session).await?;
    let (parent, depth) = H::lineage(request.parent_session_id);
    let entry = AgentEntry {
        name: request.name.to_string(),
        instructions: request.instructions.to_string(),
        session,
        parent,
        owner_session_id: request.parent_session_id.map(str::to_string),
        depth,
        model_id: Some(request.model.to_string()),
    };
    H::save_entry(request.name, &entry).await?;
    let agent_id = entry.id().to_owned();
    H::insert_reserved(entry);
    thread_status::initialize(&agent_id);
    Ok(agent_id)
}

async fn save_session<H: Registration>(name: &str, session: &H::Session) -> Result<()> {
    H::save_session(session).await.map_err(|error| {
        tracing::warn!(target: "codetether_agent::tool::agent::spawn_store", agent = %name, error = %error, "Failed to save spawned agent session");
        error
    })
}
