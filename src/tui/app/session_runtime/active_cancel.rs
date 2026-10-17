//! Direct cancellation signal shared by the TUI and session runtime.

use std::sync::Arc;

use parking_lot::Mutex;
#[cfg(test)]
use tokio::sync::Notify;

use crate::session::runtime::{
    CONTROL,
    steering::{self, SteeringInput},
};
use codetether_session_runtime::Lease;

#[path = "active_attach.rs"]
mod attach;
#[path = "active_release.rs"]
mod release;

/// Synchronized control plane for cancellation and active-turn steering.
#[derive(Clone, Default)]
pub(super) struct ActiveCancel(Arc<Mutex<Option<Lease>>>);

impl ActiveCancel {
    /// Reserve `session_id` and open its steering inbox before handoff.
    pub(super) fn prepare(&self, session_id: &str) -> bool {
        let mut active = self.0.lock();
        if active.is_some() {
            return false;
        }
        let Some(lease) = CONTROL.reserve(session_id) else {
            return false;
        };
        *active = Some(lease);
        steering::open(session_id);
        true
    }

    /// Notify the active executor, returning whether one was attached.
    pub(super) fn notify(&self) -> bool {
        self.0
            .lock()
            .as_ref()
            .is_some_and(|lease| CONTROL.cancel(lease))
    }

    /// Atomically append input when the active session still accepts it.
    pub(super) fn steer(&self, input: SteeringInput) -> bool {
        let active = self.0.lock();
        let Some(lease) = active.as_ref() else {
            return false;
        };
        CONTROL
            .with_current(lease, || steering::push(lease.session_id(), input))
            .unwrap_or(false)
    }
}

#[cfg(test)]
#[path = "active_cancel_tests.rs"]
mod tests;
