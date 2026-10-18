//! Cloneable run capabilities shared by orchestration adapters.
use crate::binding::Binding;
use std::sync::Arc;
/// Cloneable cancellation and pause capability for a shared runtime run.
///
/// Standalone handles release their reservation when the last clone is dropped.
/// Session executors attach handles to their existing lease, so child adapters
/// observe the same cancellation without becoming another owner.
///
/// # Examples
/// ```
/// use codetether_session_control::ControlHandle;
/// let handle = ControlHandle::new();
/// handle.pause();
/// assert!(handle.is_paused());
/// handle.resume();
/// handle.cancel();
/// assert!(handle.is_cancelled());
/// ```
#[derive(Clone, Debug)]
pub struct ControlHandle(pub(crate) Arc<Binding>);

#[cfg(test)]
#[path = "handle_tests.rs"]
mod tests;
