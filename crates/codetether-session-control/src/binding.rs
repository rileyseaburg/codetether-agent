//! Shared lease, delegated input log and final standalone-owner release.
use codetether_session_runtime::Lease;
use std::sync::Mutex;

pub(crate) struct Binding {
    pub(crate) lease: Lease,
    pub(crate) owned: bool,
    pub(crate) updates: Mutex<Vec<codetether_provider_core::Message>>,
    pub(crate) gate: crate::Gate,
}

impl Drop for Binding {
    fn drop(&mut self) {
        if self.owned {
            crate::CONTROL.finish(&self.lease);
        }
    }
}

impl std::fmt::Debug for Binding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Binding")
            .field("lease", &self.lease)
            .field("owned", &self.owned)
            .field("updates", &self.updates)
            .finish()
    }
}
