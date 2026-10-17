//! Atomic completion boundaries and explicit inbox cleanup.

use crate::Inboxes;

impl<T> Inboxes<T> {
    /// Close an inbox and return pending inputs so callers can release resources.
    ///
    /// # Panics
    /// Panics if a previous operation poisoned the queue mutex.
    pub fn discard(&self, session_id: &str) -> Vec<T> {
        self.queues().remove(session_id).unwrap_or_default().into()
    }

    /// Drain accepted input in FIFO order while keeping the inbox open.
    ///
    /// # Panics
    /// Panics if a previous operation poisoned the queue mutex.
    pub fn take(&self, session_id: &str) -> Vec<T> {
        self.queues()
            .get_mut(session_id)
            .map(|queue| queue.drain(..).collect())
            .unwrap_or_default()
    }

    /// Drain pending input or atomically close an empty inbox.
    ///
    /// A concurrent sender either enqueues input for this drain or receives it
    /// back from [`Self::push`]; accepted input is never lost at finalization.
    /// # Panics
    /// Panics if a previous operation poisoned the queue mutex.
    pub fn take_or_close(&self, session_id: &str) -> Vec<T> {
        let mut queues = self.queues();
        let Some(queue) = queues.get_mut(session_id) else {
            return Vec::new();
        };
        if queue.is_empty() {
            queues.remove(session_id);
            Vec::new()
        } else {
            queue.drain(..).collect()
        }
    }
}
