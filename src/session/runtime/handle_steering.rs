//! Broadcast accepted human input to every child without consuming sibling input.

use super::ControlHandle;
use crate::provider::Message;

impl ControlHandle {
    pub(crate) fn has_input(&self) -> bool {
        !self
            .0
            .updates
            .lock()
            .expect("steering log poisoned")
            .is_empty()
    }

    pub(crate) fn publish_input(&self, message: Message) {
        self.0
            .updates
            .lock()
            .expect("steering log poisoned")
            .push(message);
    }

    pub(crate) fn inputs_since(&self, cursor: &mut usize) -> Vec<Message> {
        let updates = self.0.updates.lock().expect("steering log poisoned");
        let pending = updates.get(*cursor..).unwrap_or_default().to_vec();
        *cursor = updates.len();
        pending
    }

    pub(crate) fn session_id(&self) -> Option<&str> {
        (!self.0.owned).then(|| self.0.lease.session_id())
    }
}
