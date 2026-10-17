//! Attach the TUI lifecycle notifier to shared session ownership.

use super::ActiveCancel;
use crate::session::runtime::{CONTROL, steering};
use std::sync::Arc;
use tokio::sync::Notify;

impl ActiveCancel {
    pub(in super::super) fn set(&self, session_id: &str, notify: Arc<Notify>) -> bool {
        let mut active = self.0.lock();
        if active.is_none() {
            *active = CONTROL.reserve(session_id);
        }
        let attached = active
            .as_ref()
            .filter(|lease| lease.session_id() == session_id)
            .is_some_and(|lease| CONTROL.attach(lease, notify));
        if attached {
            steering::open(session_id);
        }
        attached
    }
}
