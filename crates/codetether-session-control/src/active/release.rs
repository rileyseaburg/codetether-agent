//! Release active ownership or a rejected handoff without disturbing another turn.

use super::ActiveCancel;
use crate::CONTROL;
use codetether_session_steering as steering;

impl ActiveCancel {
    /// Clear active state and reject all later steering for that run.
    pub fn clear(&self) {
        let mut active = self.0.lock();
        if let Some(lease) = active.take() {
            CONTROL.finish_with(&lease, || steering::clear(lease.session_id()));
        }
    }

    /// Release only this session's reservation, never an attached executor.
    pub fn release_prepared(&self, session_id: &str) {
        let mut active = self.0.lock();
        if active.as_ref().is_some_and(|lease| {
            lease.session_id() == session_id
                && CONTROL.release_prepared_with(lease, || steering::clear(session_id))
        }) {
            active.take();
        }
    }
}

#[cfg(test)]
#[path = "release_tests.rs"]
mod tests;
