//! Residency-aware access to an application's existing live-agent registry.

/// Declare registry operations for an existing singleton and session type.
///
/// The visibility applies to the exported operations. Callbacks retain the
/// application's residency accounting and scoped tree resolution. The registry
/// itself is supplied by the caller, so every operation shares its existing map.
///
/// # Examples
/// ```text
/// codetether_agent_registry::declare_application_registry!(
///     pub(super), Session, REGISTRY, touch, forget, resolve_tree,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_registry {
    ($visibility:vis, $session:ty, $registry:ident, $touch:path, $forget:path, $resolve:path $(,)?) => {
        $visibility type AgentEntry = $crate::AgentEntry<$session>;
        $crate::impl_session_identity!($session);

        $visibility fn insert(entry: AgentEntry) {
            let agent_id = entry.session.id.clone();
            $touch(&agent_id);
            insert_reserved(entry);
        }

        /// Insert a child already represented by a pending residency slot.
        $visibility fn insert_reserved(entry: AgentEntry) {
            $registry.insert(entry);
        }

        $visibility fn remove(id: &str) -> Option<AgentEntry> {
            let removed = $registry.remove(id);
            $forget(id);
            removed
        }

        $visibility fn get(id: &str) -> Option<AgentEntry> {
            $registry.get(id)
        }

        $visibility fn contains_name(name: &str, owner: Option<&str>) -> bool {
            $registry.contains_name(name, owner)
        }

        /// Return whether an exact owner scope already contains the display name.
        $visibility fn contains_name_for_owner(name: &str, owner: Option<&str>) -> bool {
            $registry.contains_name_for_owner(name, owner)
        }

        $visibility fn update_session(id: &str, session: $session) {
            $registry.update_session(id, session);
        }

        $visibility fn reparent_owner(from: &str, to: &str) {
            $registry.reparent_owner(from, to);
        }

        $crate::__declare_registry_scope!($visibility, $registry, $resolve);
    };
}
