//! Stable JSON payloads consumed by A2A task observers.
//!
//! Builders serialize tool progress, text, usage and terminal markers without
//! depending on the application's session-event enum. [tool_call_complete]
//! preserves the observer's 500-character output bound.
use serde_json::{Value, json};
mod call;
mod progress;
mod status;
mod text;
mod usage;
pub use call::{tool_call_complete, tool_call_start};
pub use progress::{tool_heartbeat, tool_output_chunk};
pub use status::{done, ignored, thinking};
pub use text::{error, text_chunk, text_complete, thinking_complete};
pub use usage::usage_report;
