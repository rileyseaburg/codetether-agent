//! Live in-process approval handoff for interactive sessions.

#[path = "live/decision_event.rs"]
mod decision_event;
mod pending;
mod poll;
mod request;
mod request_build;
mod state;
mod types;

pub use request::request;
pub use state::{decide, latest_id};
pub use types::{LiveApprovalDecision, LiveApprovalRequest};

#[cfg(test)]
#[path = "live/cancellation_tests.rs"]
mod cancellation_tests;
#[cfg(test)]
#[path = "live/decision_event_tests.rs"]
mod decision_event_tests;
#[cfg(test)]
#[path = "live/external_session_tests.rs"]
mod external_session_tests;
#[cfg(test)]
#[path = "live/external_tests.rs"]
mod external_tests;
#[cfg(test)]
#[path = "live/poll_error_tests.rs"]
mod poll_error_tests;
#[cfg(test)]
mod tests;
