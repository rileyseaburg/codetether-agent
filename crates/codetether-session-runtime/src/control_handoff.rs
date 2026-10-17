//! Reserve and attach executors without losing cancellation during handoff.

use crate::{
    ActiveTurn, ControlPlane, Lease,
    control::{Entry, State},
};
use std::sync::Arc;
use tokio::sync::{Notify, watch};

impl ControlPlane {
    /// Reserve an idle session, returning `None` if it already has an owner.
    ///
    /// # Panics
    /// Panics if the control mutex is poisoned or all run generations are used.
    pub fn reserve(&self, session_id: &str) -> Option<Lease> {
        reserve(&mut self.state(), session_id)
    }

    /// Attach a frontend notifier to its reservation, delivering pending cancellation.
    ///
    /// Returns `false` for stale leases or an already attached executor.
    /// # Panics
    /// Panics if the control mutex is poisoned.
    pub fn attach(&self, lease: &Lease, notify: Arc<Notify>) -> bool {
        self.state()
            .entries
            .get_mut(lease.session_id())
            .filter(|entry| entry.lease == *lease)
            .is_some_and(|entry| entry.turn.attach(lease.session_id(), notify))
    }
}

pub(crate) fn reserve(state: &mut State, session_id: &str) -> Option<Lease> {
    if state.entries.contains_key(session_id) {
        return None;
    }
    state.next = state
        .next
        .checked_add(1)
        .expect("session run generation exhausted");
    let lease = Lease {
        session_id: session_id.into(),
        generation: state.next,
    };
    let mut turn = ActiveTurn::default();
    turn.prepare(session_id);
    let (stop, _) = watch::channel(false);
    state.entries.insert(
        session_id.into(),
        Entry {
            lease: lease.clone(),
            turn,
            stop,
            executing: false,
            paused: false,
        },
    );
    Some(lease)
}
