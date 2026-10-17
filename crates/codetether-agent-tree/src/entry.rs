//! Snapshot access required by pure agent-tree queries.
/// Minimal fields projected from a live registry entry.
///
/// # Implementors
/// Delegate access to the entry's durable ID, name and owning session.
pub trait Entry {
    /// Durable child identity.
    fn id(&self) -> &str;
    /// Name of the child within its parent.
    fn name(&self) -> &str;
    /// Owning parent session identity, if any.
    fn owner_session_id(&self) -> Option<&str>;
}
/// Source of live registry snapshots for tree operations.
///
/// # Implementors
/// Return a fresh snapshot each time the application registry is queried.
pub trait Host {
    /// Application entry wrapper used by this registry.
    type Entry: Entry;
    /// Capture all current entries in their existing iteration order.
    fn entries() -> Vec<Self::Entry>;
}
