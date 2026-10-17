//! Release active ownership or a rejected handoff without disturbing another turn.

use super::ActiveCancel;
use crate::session::runtime::{CONTROL, steering};

impl ActiveCancel {
    /// Clear active state and reject all later steering for that run.
    pub(in super::super) fn clear(&self) {
        let mut active = self.0.lock();
        if let Some(lease) = active.take() {
            CONTROL.finish_with(&lease, || steering::clear(lease.session_id()));
        }
    }

    /// Release only this session's reservation, never an attached executor.
    pub(in super::super) fn release_prepared(&self, session_id: &str) {
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
#[path = "active_release_tests.rs"]
mod tests;
