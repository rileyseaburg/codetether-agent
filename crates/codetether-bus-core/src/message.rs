//! Message declarations grouped without changing public enum variants.
use super::*;
macro_rules! finish_bus_message {
    ($($variants:tt)*) => {
        /// The set of messages the bus can carry.
        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case")]
        ///
        /// # Examples
        /// ```
        /// let message = codetether_bus_core::BusMessage::Heartbeat { agent_id: "agent".into(), status: "ready".into() };
        /// assert!(!message.is_durable());
        /// ```
        pub enum BusMessage {
            $($variants)*
        }
    };
}
include!("message/lifecycle.rs");
include!("message/tasks.rs");
include!("message/tools.rs");
include!("message/speech.rs");
include!("message/ralph.rs");
include!("message/observability.rs");
include!("message/voice.rs");
bus_variants_0! {}
