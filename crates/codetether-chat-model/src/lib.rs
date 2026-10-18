//! Serializable chat messages and retained raw payload policy.
//!
//! [ChatMessage] combines [MessageType], content, timestamp and optional
//! [MessageUsage]. [copy] selects raw clipboard payloads; [payload] bounds
//! tool output without depending on terminal rendering or session storage.
pub mod copy;
mod message;
mod message_type;
pub mod payload;
mod usage;
pub use message::ChatMessage;
pub use message_type::MessageType;
pub use usage::MessageUsage;

mod streaming_clock;
mod streaming_state;
mod streaming_text;
pub use streaming_state::StreamingState;
