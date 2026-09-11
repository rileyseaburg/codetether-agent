//! Live in-process approval handoff for interactive sessions.

mod request;
mod request_arguments;
mod request_build;
mod state;
#[path = "live/state_query.rs"]
mod state_query;
mod types;

pub use request::request;
pub use state::decide;
pub(crate) use state_query::is_pending;
pub use state_query::latest_id;
pub use types::{LiveApprovalDecision, LiveApprovalRequest};

#[cfg(test)]
#[path = "live/order_tests.rs"]
mod order_tests;
#[cfg(test)]
mod tests;
