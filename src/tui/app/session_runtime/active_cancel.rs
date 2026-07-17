//! Direct cancellation signal shared by the TUI and session runtime.

use std::sync::Arc;

use parking_lot::Mutex;
use tokio::sync::Notify;

use super::active_turn::ActiveTurn;
use crate::session::helper::steering::SteeringInput;

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

    /// Clear active state and reject all later steering for that run.
    pub(super) fn clear(&self) {
        let mut active = self.0.lock();
        if let Some(session_id) = active.clear() {
            crate::session::helper::steering::clear(&session_id);
        }
    }

    /// Notify the active executor, returning whether one was attached.
    pub(super) fn notify(&self) -> bool {
        if let Some(notify) = self.0.lock().cancel() {
            notify.notify_one();
            return true;
        }
        false
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
