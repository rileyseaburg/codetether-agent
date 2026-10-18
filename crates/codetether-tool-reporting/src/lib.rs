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
