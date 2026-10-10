use crate::Session;
use std::collections::HashMap;

/// Bounded session registry with a global 30-per-minute pairing budget.
///
/// It retains at most four sessions. Raw device tokens are returned once,
/// never stored. Keep one registry per relay process, behind one serialized
/// mutation boundary; recreating it per request defeats its limits.
///
/// ```
/// let registry = codetether_companion_core::Registry::default();
/// assert_eq!(registry.len(), 0);
/// ```
#[derive(Default)]
pub struct Registry {
    pub(crate) sessions: HashMap<String, Session>,
    pub(crate) attempts: u32,
    pub(crate) window: i64,
}
impl Registry {
    /// Number of entries, including ended entries until swept.
    pub fn len(&self) -> usize {
        self.sessions.len()
    }
    /// Whether the registry has no entries.
    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }
}
