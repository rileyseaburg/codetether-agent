//! Non-human delegated message handling.

use crate::provider::Message;
use crate::session::Session;

impl Session {
    /// Record a delegated message without allowing it to elevate context access.
    pub(crate) fn add_delegated_message(&mut self, message: Message) {
        super::session_update::delegated(self, &message);
        self.add_message(message);
    }
}
