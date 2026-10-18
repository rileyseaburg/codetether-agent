//! Construction and ownership binding for runtime capabilities.

use crate::{ControlHandle, binding::Binding};
use codetether_session_runtime::Lease;
use std::sync::Arc;

impl Default for ControlHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlHandle {
    /// Create a standalone run controlled by the shared runtime.
    /// # Panics
    /// Panics if the ownership mutex is poisoned or run identities are exhausted.
    /// # Returns
    /// A new standalone control handle that releases its lease on final drop.
    /// # Examples
    /// ```rust
    /// let handle = codetether_session_control::ControlHandle::new();
    /// assert!(!handle.is_cancelled());
    /// ```
    pub fn new() -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let lease = crate::CONTROL
            .reserve(&id)
            .expect("fresh standalone runtime identity");
        Self(Arc::new(Binding {
            lease,
            owned: true,
            gate: crate::gate::unrestricted,
            updates: Default::default(),
        }))
    }

    pub(crate) fn attached(lease: Lease, gate: crate::Gate) -> Self {
        Self(Arc::new(Binding {
            lease,
            owned: false,
            gate,
            updates: Default::default(),
        }))
    }
}
