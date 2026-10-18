//! Worker SSE task decoding and bus-event forwarding.
//!
//! [SseBuffer] bounds partial frames and produces [IncomingTask] values.
//! [forward_bus_event] sends only supported observability events through a
//! caller-owned HTTP client; bridge lifecycle remains with the caller.
mod agent_payload;
mod buffer;
mod error;
mod event;
mod forward;
mod payload;
mod task;
pub use buffer::SseBuffer;
pub use error::Error;
pub use forward::forward_bus_event;
pub use task::IncomingTask;
