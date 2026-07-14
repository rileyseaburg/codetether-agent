//! Trusted human-ingress message handling.

use crate::provider::Message;
use crate::session::Session;

impl Session {
    /// Record a trusted human message and append it to the transcript.
    pub(crate) fn add_human_message(&mut self, message: Message) {
        super::session_update::trusted(self, &message);
        self.add_message(message);
    }
}
