//! Owner-filtered snapshots and creation lineage.
use crate::{AgentEntry, Registry, Session};
impl<S: Session> Registry<S> {
    /// Clone entries for an owner, or every entry when the owner is absent.
    /// # Arguments
    /// * `parent` — Owning session ID, or none for a full snapshot.
    /// # Returns
    /// Matching entries in the map's existing iteration order.
    /// # Examples
    /// See [Registry] for construction.
    pub fn entries_for_parent(&self, parent: Option<&str>) -> Vec<AgentEntry<S>> {
        self.entries
            .read()
            .values()
            .filter(|entry| parent.is_none() || entry.owner_session_id.as_deref() == parent)
            .cloned()
            .collect()
    }
    /// Derive the parent display name and next creation depth.
    /// # Arguments
    /// * `session_id` — Optional parent session ID.
    /// # Returns
    /// No session gives depth zero; an unknown session gives depth one;
    /// a registered parent gives its name and saturating depth plus one.
    /// # Examples
    /// See [Registry] for construction.
    pub fn lineage_for_session(&self, session_id: Option<&str>) -> (Option<String>, u8) {
        let Some(session_id) = session_id else {
            return (None, 0);
        };
        self.entries
            .read()
            .get(session_id)
            .map_or((None, 1), |entry| {
                (Some(entry.name.clone()), entry.depth.saturating_add(1))
            })
    }
}
