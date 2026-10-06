//! Direct cancellation signal shared by the TUI and session runtime.

use std::sync::Arc;

use parking_lot::Mutex;
use tokio::sync::Notify;

use super::active_turn::ActiveTurn;
use crate::session::helper::steering::SteeringInput;

#[path = "active_release.rs"]
mod release;

/// Synchronized control plane for cancellation and active-turn steering.
#[derive(Clone, Default)]
pub(super) struct ActiveCancel(Arc<Mutex<ActiveTurn>>);

impl ActiveCancel {
    /// Reserve `session_id` and open its steering inbox before handoff.
    pub(super) fn prepare(&self, session_id: &str) -> bool {
        let mut active = self.0.lock();
        if !active.prepare(session_id) {
            return false;
        }
        crate::session::helper::steering::open(session_id);
        true
    }

    /// Attach a runtime cancellation notifier to the named session.
    pub(super) fn set(&self, session_id: &str, notify: Arc<Notify>) -> bool {
        let mut active = self.0.lock();
        let attached = active.attach(session_id, notify);
        if attached {
            crate::session::helper::steering::open(session_id);
        }
        attached
    }

    /// Notify the active executor, returning whether one was attached.
    pub(super) fn notify(&self) -> bool {
        self.0.lock().request_cancel()
    }

    /// Atomically append input when the active session still accepts it.
    pub(super) fn steer(&self, input: SteeringInput) -> bool {
        let active = self.0.lock();
        let Some(session_id) = active.session_id() else {
            return false;
        };
        crate::session::helper::steering::push(session_id, input)
    }
}

#[cfg(test)]
#[path = "active_cancel_tests.rs"]
mod tests;
