//! Synchronized ownership of the live-agent map.
use crate::AgentEntry;
use parking_lot::RwLock;
use std::collections::HashMap;

/// Live agents keyed by durable session identity.
/// # Examples
/// ```rust
/// use codetether_agent_registry::Registry;
/// let _registry: Registry<String> = Registry::default();
/// ```
pub struct Registry<S> {
    pub(crate) entries: RwLock<HashMap<String, AgentEntry<S>>>,
}
impl<S> Default for Registry<S> {
    fn default() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }
}
