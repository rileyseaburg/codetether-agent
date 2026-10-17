//! Cloneable run capabilities shared by all orchestration adapters.

use codetether_session_runtime::Lease;
use std::sync::Arc;
use std::sync::Mutex;

#[path = "handle_new.rs"]
mod construction;
#[path = "handle_ops.rs"]
mod operations;
#[path = "handle_ready.rs"]
mod readiness;
#[path = "handle_scope.rs"]
mod scope;
#[path = "handle_steering.rs"]
mod steering;

#[derive(Debug)]
struct Binding {
    lease: Lease,
    owned: bool,
    updates: Mutex<Vec<crate::provider::Message>>,
}

impl Drop for Binding {
    fn drop(&mut self) {
        if self.owned {
            super::CONTROL.finish(&self.lease);
        }
    }
}

/// Cloneable cancellation and pause capability for a shared runtime run.
///
/// Standalone handles release their reservation when the last clone is dropped.
/// Session executors attach handles to their existing lease, so child adapters
/// observe the same cancellation without becoming another owner.
///
/// # Examples
/// ```
/// use codetether_agent::session::runtime::ControlHandle;
/// let handle = ControlHandle::new();
/// handle.pause();
/// assert!(handle.is_paused());
/// handle.resume();
/// handle.cancel();
/// assert!(handle.is_cancelled());
/// ```
#[derive(Clone, Debug)]
pub struct ControlHandle(Arc<Binding>);

#[cfg(test)]
#[path = "handle_tests.rs"]
mod tests;
