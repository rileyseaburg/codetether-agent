//! Terminal task updates and correlated response messages.
use super::*;
impl Settle<'_> {
    /// Mark the task completed with `text` and return the response message.
    pub fn completed(&self, text: String) -> Message {
        let message = complete(self.tasks, self.task_id, self.context_id, text.clone());
        self.telemetry(true, Some(text), None);
        message
    }

    /// Mark the task failed and return the error message.
    pub fn failed(&self, error: &anyhow::Error) -> Message {
        tracing::error!(target: "codetether_agent::a2a::server::server_settle",task_id = %self.task_id, %error, "Inbound A2A turn failed");
        let message = fail(self.tasks, self.task_id, self.context_id, error);
        self.telemetry(false, None, Some(error.to_string()));
        message
    }
}
