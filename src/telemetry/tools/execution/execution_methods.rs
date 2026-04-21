//! Builder and finalization methods for [`super::ToolExecution`].

use chrono::Utc;

use super::super::FileChange;
use super::ToolExecution;

impl ToolExecution {
    /// Start a new pending execution. `success` defaults to `false` until
    /// explicitly completed.
    pub fn start(tool_name: &str, input: serde_json::Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            tool_name: tool_name.to_string(),
            timestamp: Utc::now(),
            duration_ms: 0,
            success: false,
            error: None,
            tokens_used: None,
            session_id: None,
            input: Some(input),
            file_changes: Vec::new(),
        }
    }

    /// Attach a [`FileChange`] the tool performed.
    pub fn add_file_change(&mut self, change: FileChange) {
        self.file_changes.push(change);
    }

    /// Builder-style: associate this execution with a session.
    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// In-place finalizer — mutates an existing record.
    pub fn complete(&mut self, success: bool, duration_ms: u64) {
        self.success = success;
        self.duration_ms = duration_ms;
    }

    /// In-place failure finalizer.
    pub fn fail(&mut self, error: String, duration_ms: u64) {
        self.success = false;
        self.error = Some(error);
        self.duration_ms = duration_ms;
    }

    /// Consuming finalizer for success. `_output` is accepted for API
    /// compatibility and currently discarded.
    pub fn complete_success(mut self, _output: String, duration: std::time::Duration) -> Self {
        self.success = true;
        self.duration_ms = duration.as_millis() as u64;
        self
    }

    /// Consuming finalizer for failure.
    pub fn complete_error(mut self, error: String, duration: std::time::Duration) -> Self {
        self.success = false;
        self.error = Some(error);
        self.duration_ms = duration.as_millis() as u64;
        self
    }
}
