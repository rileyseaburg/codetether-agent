//! Message-window offsets and counts from the instance-local checkpoint.
use super::State;

impl State {
    /// Return the absolute offset of the first loaded message.
    /// # Returns
    /// The number of durable records preceding the loaded window.
    /// # Panics
    /// Panics if the checkpoint mutex is poisoned, preserving the session contract.
    /// # Examples
    /// ```
    /// assert_eq!(codetether_session_store::State::default().message_offset(), 0);
    /// ```
    pub fn message_offset(&self) -> usize {
        self.0.lock().unwrap().message_start
    }

    /// Count durable history plus the current loaded message tail.
    /// # Arguments
    /// `loaded` is the number of messages currently held by the session.
    /// # Returns
    /// The larger of the loaded window's end and the known durable end.
    /// # Panics
    /// Panics if the checkpoint mutex is poisoned, preserving the session contract.
    /// # Examples
    /// ```
    /// assert_eq!(codetether_session_store::State::default().message_count(3), 3);
    /// ```
    pub fn message_count(&self, loaded: usize) -> usize {
        let state = self.0.lock().unwrap();
        (state.message_start + loaded).max(state.message_end)
    }
}
