//! Synchronized storage for session-addressed steering input.

use std::collections::{HashMap, VecDeque};
use std::sync::{Mutex, MutexGuard};

type Queues<T> = HashMap<String, VecDeque<T>>;

/// FIFO inboxes with an atomic drain-or-close completion boundary.
///
/// The runtime owns queue state; callers own input representation and delivery.
/// Reopening an existing inbox preserves pending input.
///
/// # Examples
/// ```
/// use codetether_session_runtime::Inboxes;
/// let inboxes = Inboxes::default();
/// assert_eq!(inboxes.push("idle", 1), Err(1));
/// inboxes.open("active");
/// assert!(inboxes.push("active", 2).is_ok());
/// assert_eq!(inboxes.discard("active"), [2]);
/// ```
pub struct Inboxes<T>(Mutex<Queues<T>>);

impl<T> Default for Inboxes<T> {
    fn default() -> Self {
        Self(Mutex::new(HashMap::new()))
    }
}

impl<T> Inboxes<T> {
    /// Open an inbox, preserving inputs already accepted during handoff.
    ///
    /// # Panics
    /// Panics if a previous operation poisoned the queue mutex.
    pub fn open(&self, session_id: &str) {
        self.queues().entry(session_id.to_string()).or_default();
    }

    /// Accept input only while open, returning rejected input to the caller.
    ///
    /// # Errors
    /// Returns the original input if the session has no open inbox.
    /// # Panics
    /// Panics if a previous operation poisoned the queue mutex.
    pub fn push(&self, session_id: &str, input: T) -> Result<(), T> {
        let mut queues = self.queues();
        let Some(queue) = queues.get_mut(session_id) else {
            return Err(input);
        };
        queue.push_back(input);
        Ok(())
    }

    pub(crate) fn queues(&self) -> MutexGuard<'_, Queues<T>> {
        self.0.lock().expect("steering inbox mutex poisoned")
    }
}
