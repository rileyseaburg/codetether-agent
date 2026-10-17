//! Record the settled inbound turn through shared process telemetry.
use super::*;
impl Settle<'_> {
    pub(super) fn telemetry(&self, success: bool, text: Option<String>, error: Option<String>) {
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
