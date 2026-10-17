//! Process-local ownership table for all frontend executors.

use crate::{ActiveTurn, Lease};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use tokio::sync::watch;

pub(crate) struct Entry {
    pub lease: Lease,
    pub turn: ActiveTurn,
    pub stop: watch::Sender<bool>,
    pub executing: bool,
    pub paused: bool,
}

#[derive(Default)]
pub(crate) struct State {
    pub next: u64,
    pub entries: HashMap<String, Entry>,
}

/// Shared executor ownership and cancellation, keyed by durable session ID.
///
/// Frontends reserve and attach; executors claim the reservation with
/// [`Self::begin`]. A [`Lease`] identifies one generation, so stale cleanup
/// cannot release a replacement run. Inboxes and durable goal journals are
/// adapted by the application without depending on any frontend.
///
/// # Examples
/// ```
/// use codetether_session_runtime::ControlPlane;
/// let control = ControlPlane::default();
/// let lease = control.reserve("session").unwrap();
/// assert!(control.cancel(&lease));
/// let execution = control.begin("session").unwrap();
/// assert_eq!(execution.lease(), &lease);
/// assert!(control.finish(&lease));
/// ```
#[derive(Default)]
pub struct ControlPlane(Mutex<State>);

impl ControlPlane {
    pub(crate) fn state(&self) -> MutexGuard<'_, State> {
        self.0.lock().expect("session control mutex poisoned")
    }
}
