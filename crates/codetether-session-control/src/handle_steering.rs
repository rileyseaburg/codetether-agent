//! Broadcast accepted human input to every child without consuming sibling input.

use crate::ControlHandle;
use codetether_provider_core::Message;

impl ControlHandle {
    /// Return whether any human input has been published to this run.
    /// # Panics
    /// Panics if the input-log mutex is poisoned.
    pub fn has_input(&self) -> bool {
        !self
            .0
            .updates
            .lock()
            .expect("steering log poisoned")
            .is_empty()
    }

    /// Append human input for every child reader's independent cursor.
    /// # Arguments
    /// `message` is the accepted input to broadcast.
    /// # Panics
    /// Panics if the input-log mutex is poisoned.
    pub fn publish_input(&self, message: Message) {
        self.0
            .updates
            .lock()
            .expect("steering log poisoned")
            .push(message);
    }

    /// Clone new inputs and advance only the supplied reader's cursor.
    /// # Arguments
    /// `cursor` is this reader's offset into the shared input log.
    /// # Returns
    /// Inputs published since the offset, in publication order.
    /// # Panics
    /// Panics if the input-log mutex is poisoned.
    pub fn inputs_since(&self, cursor: &mut usize) -> Vec<Message> {
        let updates = self.0.updates.lock().expect("steering log poisoned");
        let pending = updates.get(*cursor..).unwrap_or_default().to_vec();
        *cursor = updates.len();
        pending
    }

    /// Return the durable session identity of an attached handle.
    /// # Returns
    /// `None` for standalone handles and the lease's session ID otherwise.
    pub fn session_id(&self) -> Option<&str> {
        (!self.0.owned).then(|| self.0.lease.session_id())
    }
}
