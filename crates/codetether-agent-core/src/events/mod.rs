//! Display-independent agent message and tool-call event payloads.
//!
//! AgentMessageEntry carries message text and role; AgentToolCallDetail carries
//! tool previews and outcome. Both can be consumed by any presentation layer.
mod message;
mod tool_call;
pub use message::AgentMessageEntry;
pub use tool_call::AgentToolCallDetail;
