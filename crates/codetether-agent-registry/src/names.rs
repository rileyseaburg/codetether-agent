//! Display-name existence checks with wildcard or exact owner scope.
use crate::{Registry, Session};
impl<S: Session> Registry<S> {
    /// Check a display name, treating an absent owner as all scopes.
    /// # Arguments
    /// * `name` — Exact display name.
    /// * `owner` — Exact owner, or none to search every owner.
    /// # Returns
    /// Whether any matching live entry exists.
    /// # Examples
    /// See [Registry] for construction.
    pub fn contains_name(&self, name: &str, owner: Option<&str>) -> bool {
        self.entries.read().values().any(|entry| {
            entry.name == name && (owner.is_none() || entry.owner_session_id.as_deref() == owner)
        })
    }
    /// Check a display name in one exact owner scope, including unowned agents.
    /// # Arguments
    /// * `name` — Exact display name.
    /// * `owner` — Owner ID; none matches only entries without an owner.
    /// # Returns
    /// Whether any matching live entry exists.
    /// # Examples
    /// See [Registry] for construction.
    pub fn contains_name_for_owner(&self, name: &str, owner: Option<&str>) -> bool {
        self.entries
            .read()
            .values()
            .any(|entry| entry.name == name && entry.owner_session_id.as_deref() == owner)
    }
}
