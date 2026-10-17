//! Construction and ownership binding for runtime capabilities.

use super::{Binding, ControlHandle};
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
    pub fn new() -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let lease = crate::session::runtime::CONTROL
            .reserve(&id)
            .expect("fresh standalone runtime identity");
        Self(Arc::new(Binding {
            lease,
            owned: true,
            updates: Default::default(),
        }))
    }

    pub(crate) fn attached(lease: Lease) -> Self {
        Self(Arc::new(Binding {
            lease,
            owned: false,
            updates: Default::default(),
        }))
    }
}
