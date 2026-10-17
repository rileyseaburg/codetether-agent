//! SubAgent lifecycle operations.
use super::*;
impl SubAgent {
    /// Record a tool call
    ///
    /// # Arguments
    /// * `name` - Value used for this operation.
    /// * `success` - Value used for this operation.
    /// # Returns
    /// Unit after adding a tool call and advancing activity counters.
    pub fn record_tool_call(&mut self, name: &str, success: bool) {
        self.tool_calls.push(ToolCallRecord {
            name: name.to_string(),
            timestamp: Utc::now(),
            success,
        });
        self.steps += 1;
        self.last_active = Utc::now();
    }

    /// Append to output
    ///
    /// # Arguments
    /// * `text` - Value used for this operation.
    /// # Returns
    /// Unit after appending text and updating activity time.
    pub fn append_output(&mut self, text: &str) {
        self.output.push_str(text);
        self.last_active = Utc::now();
    }

    /// Set status
    ///
    /// # Arguments
    /// * `status` - Value used for this operation.
    /// # Returns
    /// Unit after changing status and updating activity time.
    pub fn set_status(&mut self, status: SubAgentStatus) {
        self.status = status;
        self.last_active = Utc::now();
    }
}
