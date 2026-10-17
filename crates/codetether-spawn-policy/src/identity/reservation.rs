//! RAII guard for one pending owner-scoped child name.

use super::registry::{Key, Registry};
use std::sync::Arc;

/// Pending owner/name claim that releases itself unless committed.
/// # Examples
/// Obtain a claim using [crate::reserve]; dropping it rolls back admission.
/// Call [Self::commit] only after the durable store owns the identity.
pub struct Reservation {
    registry: Arc<Registry>,
    key: Key,
    active: bool,
}

impl Reservation {
    pub(super) fn try_with(
        registry: Arc<Registry>,
        name: &str,
        owner: Option<&str>,
    ) -> Option<Self> {
        let key = registry.claim(name, owner)?;
        Some(Self {
            registry,
            key,
            active: true,
        })
    }

    /// Complete the handoff after the durable store owns the identity.
    pub fn commit(mut self) {
        self.registry.release(&self.key);
        self.active = false;
    }
}

impl Drop for Reservation {
    fn drop(&mut self) {
        if self.active {
            self.registry.release(&self.key);
        }
    }
}
