//! Release active ownership or a rejected handoff without disturbing another turn.

use super::ActiveCancel;
use crate::session::helper::steering;

impl ActiveCancel {
    /// Clear active state and reject all later steering for that run.
    pub(in super::super) fn clear(&self) {
        let mut active = self.0.lock();
        if let Some(session_id) = active.clear() {
            steering::clear(&session_id);
        }
    }

    /// Release only this session's reservation, never an attached executor.
    pub(in super::super) fn release_prepared(&self, session_id: &str) {
        let mut active = self.0.lock();
        if active.session_id() == Some(session_id) && active.cancel().is_none() {
            active.clear();
            steering::clear(session_id);
        }
    }
}

#[cfg(test)]
#[path = "active_release_tests.rs"]
mod tests;
