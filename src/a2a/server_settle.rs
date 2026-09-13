//! Recording the outcome of an inbound `message/send` on the task table.

#[path = "server_settle_outcome.rs"]
mod outcome;

use std::time::Duration;

use dashmap::DashMap;

use crate::a2a::types::{Message, Task};

use super::server_telemetry::record_a2a_message_telemetry;

pub(super) struct Settle<'a> {
    pub tasks: &'a DashMap<String, Task>,
    pub task_id: &'a str,
    pub context_id: Option<&'a str>,
    pub prompt: &'a str,
    pub blocking: bool,
    pub elapsed: Duration,
}

impl Settle<'_> {
    /// Mark the task completed with `text` and return the response message.
    pub(super) fn completed(&self, text: String) -> Message {
        let message = outcome::complete(self.tasks, self.task_id, self.context_id, text.clone());
        self.telemetry(true, Some(text), None);
        message
    }

    /// Mark the task failed and return the error message.
    pub(super) fn failed(&self, error: &anyhow::Error) -> Message {
        tracing::error!(task_id = %self.task_id, %error, "Inbound A2A turn failed");
        let message = outcome::fail(self.tasks, self.task_id, self.context_id, error);
        self.telemetry(false, None, Some(error.to_string()));
        message
    }

    fn telemetry(&self, success: bool, text: Option<String>, error: Option<String>) {
        record_a2a_message_telemetry(
            "a2a_message_send",
            self.task_id,
            self.blocking,
            self.prompt,
            self.elapsed,
            success,
            text,
            error,
        );
    }
}
