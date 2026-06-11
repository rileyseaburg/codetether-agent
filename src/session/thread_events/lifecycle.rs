//! Turn lifecycle mapping.

use serde_json::json;

use crate::session::thread_store::ThreadEvent;

use super::mapper::ThreadEventMapper;

impl ThreadEventMapper {
    /// Map the beginning of a prompt turn.
    pub fn turn_started(&mut self, message: &str) -> ThreadEvent {
        self.event("turn.started", json!({ "message": message }))
    }

    /// Map successful completion of a prompt turn.
    pub fn turn_completed(&mut self, response: &str) -> ThreadEvent {
        self.event("turn.completed", json!({ "response": response }))
    }

    /// Map failed completion of a prompt turn.
    pub fn turn_failed(&mut self, error: &str) -> ThreadEvent {
        self.event("turn.failed", json!({ "error": error }))
    }
}
