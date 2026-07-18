//! Session prompt result and streaming event facade.
//!
//! [`SessionResult`] is the terminal value returned from a successful prompt.
//! [`SessionEvent`] is streamed to UIs and durable sinks for live progress,
//! usage accounting, compaction telemetry, and error reporting.

mod durable;
mod result;
mod stream_retry;
mod types;

pub use result::SessionResult;
pub use stream_retry::StreamRetryEvent;
pub use types::SessionEvent;

#[cfg(test)]
mod durable_tests;
