//! Live in-process approval handoff for interactive sessions.

mod request;
mod request_build;
mod state;
mod types;

pub use request::request;
pub use state::{decide, latest_id};
pub use types::{LiveApprovalDecision, LiveApprovalRequest};

#[cfg(test)]
mod tests;
