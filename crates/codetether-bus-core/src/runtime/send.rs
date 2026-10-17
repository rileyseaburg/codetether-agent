//! send operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Send a message on the given topic.
    pub fn send(&self, topic: impl Into<String>, message: BusMessage) -> usize {
        self.send_with_correlation(topic, message, None)
    }

    /// Send a message with a correlation id.
    pub fn send_with_correlation(
        &self,
        topic: impl Into<String>,
        message: BusMessage,
        correlation_id: Option<String>,
    ) -> usize {
        publication::publish(
            &self.bus,
            &self.agent_id,
            topic.into(),
            message,
            correlation_id,
        )
    }
}
