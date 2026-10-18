//! Tool execution reporting independent of native session ownership.
//!
//! [start], [complete] and [send_metadata] emit typed session events.
//! [tool_success_detail] and [tool_failure_detail] produce bounded audit JSON
//! with output hashes. These helpers do not execute tools or record audit logs.
mod detail;
mod digest;
mod events;
mod metadata;
pub use detail::{tool_failure_detail, tool_success_detail};
pub use events::{complete, start};
pub use metadata::send as send_metadata;
#[cfg(test)]
mod tests;

mod audit;
mod error_text;
mod failure;
mod heartbeat;
mod heartbeat_guard;
mod progress;
mod tuple;
mod unknown;
pub use audit::record_audit;
pub use error_text::diagnostic;
pub use failure::failure;
pub use heartbeat::spawn_heartbeat;
pub use heartbeat_guard::ToolHeartbeat;
pub use progress::progress_input;
pub use tuple::ToolTuple;
pub use unknown::unknown_tool;
#[cfg(test)]
mod error_tests;
#[cfg(test)]
mod heartbeat_tests;

mod registration;
pub use registration::register_progress;
