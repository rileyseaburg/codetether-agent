//! Tree-aware scoped lookup with unambiguous display-name fallback.
use crate::{AgentEntry, Registry, Session};
impl<S: Session> Registry<S> {
    /// Resolve a tree path, canonical ID, or unique display name in order.
    /// The tree callback runs before a registry lock is acquired so it may
    /// safely obtain its own registry snapshot.
    /// # Arguments
    /// * `target` — Path, durable ID, or display name.
    /// * `parent` — Owner scope; none allows all owners.
    /// * `resolve` — Tree resolver, invoked only when an owner is present.
    /// # Returns
    /// The canonical ID and cloned entry, or none for a missing/ambiguous target.
    /// # Examples
    /// See [Registry] for construction.
    pub fn resolve_for_parent(
        &self,
        target: &str,
        parent: Option<&str>,
        resolve: impl FnOnce(&str, &str) -> Option<String>,
    ) -> Option<(String, AgentEntry<S>)> {
        if let Some(parent) = parent
            && let Some(id) = resolve(parent, target)
            && let Some(entry) = self.entries.read().get(&id).cloned()
        {
            return Some((id, entry));
        }
        let store = self.entries.read();
        if let Some(entry) = store.get(target)
            && (parent.is_none() || entry.owner_session_id.as_deref() == parent)
        {
            return Some((target.into(), entry.clone()));
        }
        let mut matches = store.iter().filter(|(_, entry)| {
            entry.name == target
                && (parent.is_none() || entry.owner_session_id.as_deref() == parent)
        });
        let first = matches
            .next()
            .map(|(id, entry)| (id.clone(), entry.clone()));
        if matches.next().is_some() {
            None
        } else {
            first
        }
    }
}
