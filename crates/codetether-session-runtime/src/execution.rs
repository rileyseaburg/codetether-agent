//! Cancellation subscription for a claimed session execution.

use crate::Lease;
use tokio::sync::watch;

/// Run identity and persistent cancellation signal for an executor or observer.
///
/// Created by [`crate::ControlPlane::begin`] or [`crate::ControlPlane::subscribe`].
/// The executor owner must finish the lease on completion or drop via its
/// execution guard; subscribers do not acquire ownership. Cancellation remains
/// observable by every subscriber, including cancellation requested at handoff.
///
/// # Examples
/// ```
/// use codetether_session_runtime::ControlPlane;
/// let runtime = ControlPlane::default();
/// let execution = runtime.begin("session").unwrap();
/// assert!(runtime.cancel(execution.lease()));
/// assert!(runtime.finish(execution.lease()));
/// ```
pub struct Execution {
    pub(crate) lease: Lease,
    pub(crate) stop: watch::Receiver<bool>,
}

impl Execution {
    /// Identity used for cancellation and generation-checked cleanup.
    pub fn lease(&self) -> &Lease {
        &self.lease
    }

    /// Wait for cancellation, including a request made before this subscription.
    ///
    /// This future also resolves if the ownership table releases the execution.
    pub async fn cancelled(&mut self) {
        while !*self.stop.borrow_and_update() {
            if self.stop.changed().await.is_err() {
                return;
            }
        }
    }
}
