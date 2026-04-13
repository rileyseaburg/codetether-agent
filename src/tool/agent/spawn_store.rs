//! Persistence helpers for spawned agents.
//!
//! This module saves newly created sessions and registers them in the
//! in-memory agent store. It keeps storage side effects separate from
//! spawn validation and request parsing.
//!
//! # Examples
//!
//! ```ignore
//! persist_spawned_agent(name, instructions, session).await;
//! ```

use super::store::{self, AgentEntry};
use crate::session::Session;

/// Saves a spawned agent session and registers it in the in-memory store.
///
/// # Examples
///
/// ```ignore
/// persist_spawned_agent("reviewer", "Audit code", session).await;
/// ```
pub(super) async fn persist_spawned_agent(name: &str, instructions: &str, session: Session) {
    save_session(name, &session).await;
    store::insert(
        name.to_string(),
        AgentEntry {
            instructions: instructions.to_string(),
            session,
        },
    );
}

async fn save_session(name: &str, session: &Session) {
    if let Err(error) = session.save().await {
        tracing::warn!(agent = %name, error = %error, "Failed to save spawned agent session");
    }
}
