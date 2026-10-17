//! Structured worker tool and reasoning events.
//!
//! Event constructors bound output and redact sensitive argument keys. [sink]
//! holds the active callback and emits both typed events and legacy text.
use serde_json::{Map, Value};
mod limits;
use limits::SENSITIVE_KEYS;
pub use limits::{MAX_ARG_BYTES, MAX_OUTPUT_BYTES};
mod clamp;
mod is_sensitive_key;
mod reasoning_event;
mod redact_arguments;
mod tool_call_event;
mod tool_result_event;
use clamp::clamp;
pub use is_sensitive_key::is_sensitive_key;
pub use reasoning_event::reasoning_event;
pub use redact_arguments::redact_arguments;
pub use tool_call_event::tool_call_event;
pub use tool_result_event::tool_result_event;
pub mod sink;
#[cfg(test)]
mod tests;
