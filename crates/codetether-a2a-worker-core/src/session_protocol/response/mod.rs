//! Worker model-response sequencing over host-owned effects.
//!
//! [process_response] records usage, publishes text, appends the assistant
//! message, then runs tool calls through [ResponseHost] in response order.
mod host;
mod process;
pub use host::ResponseHost;
pub use process::process_response;
