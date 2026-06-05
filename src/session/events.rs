//! Session prompt result and streaming event facade.
//!
//! [`SessionResult`] is the terminal value returned from a successful prompt.
//! [`SessionEvent`] is streamed to UIs and durable sinks for live progress,
//! usage accounting, compaction telemetry, and error reporting.

mod durable;
mod result;
mod types;

pub use result::SessionResult;
pub use types::SessionEvent;
