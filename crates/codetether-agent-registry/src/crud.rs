//! Canonical-ID reads and mutations.
use crate::{AgentEntry, Registry, Session};
impl<S: Session> Registry<S> {
    /// Insert or replace an entry. The caller manages residency accounting.
    /// # Arguments
    /// * `entry` — Live agent keyed by its session ID.
    /// # Returns
    /// Unit after insertion.
    /// # Examples
    /// See [AgentEntry] for constructing an entry.
    pub fn insert(&self, entry: AgentEntry<S>) {
        self.entries.write().insert(entry.id().to_owned(), entry);
    }
    /// Remove an ID and return its entry, if present.
    /// # Arguments
    /// * `id` — Durable session ID.
    /// # Returns
    /// The removed entry, or none.
    /// # Examples
    /// See [Registry] for construction.
    pub fn remove(&self, id: &str) -> Option<AgentEntry<S>> {
        self.entries.write().remove(id)
    }
    /// Clone an entry by durable session ID.
    /// # Arguments
    /// * `id` — Durable session ID.
    /// # Returns
    /// A snapshot of the entry, or none.
    /// # Examples
    /// See [Registry] for construction.
    pub fn get(&self, id: &str) -> Option<AgentEntry<S>> {
        self.entries.read().get(id).cloned()
    }
    /// Replace the session of an existing entry; missing IDs are ignored.
    /// # Arguments
    /// * `id` — Existing registry key.
    /// * `session` — Replacement session, without rekeying the entry.
    /// # Returns
    /// Unit after the update.
    /// # Examples
    /// See [AgentEntry] for a session implementation.
    pub fn update_session(&self, id: &str, session: S) {
        if let Some(entry) = self.entries.write().get_mut(id) {
            entry.session = session;
        }
    }
    /// Change every exact matching owner from `from` to `to`.
    /// # Arguments
    /// * `from` — Previous owner ID.
    /// * `to` — Replacement owner ID.
    /// # Returns
    /// Unit after updating matching entries.
    /// # Examples
    /// See [Registry] for construction.
    pub fn reparent_owner(&self, from: &str, to: &str) {
        for entry in self.entries.write().values_mut() {
            if entry.owner_session_id.as_deref() == Some(from) {
                entry.owner_session_id = Some(to.to_string());
            }
        }
    }
}
