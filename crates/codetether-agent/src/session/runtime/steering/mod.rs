//! Native transcript binding for the shared session-steering crate.
//!
//! The crate owns inboxes, activity tracking, input conversion and local sockets.
//! This facade retains application API paths and native transcript publication.
mod drain;
codetether_session_steering::impl_session_transcript!(
    crate::session::Session,
    crate::session::helper::publish_user_prompt::publish
);
pub use codetether_session_steering::send;
pub(crate) use codetether_session_steering::{SteeringInput, clear, open, push};
#[cfg(not(test))]
pub(crate) use drain::drain_into;
#[cfg(test)]
#[cfg(not(test))]
pub(crate) use drain::drain_or_close_into;
#[cfg(test)]
#[path = "steering_activity_tests.rs"]
mod activity_tests;
#[cfg(test)]
mod tests;

#[cfg(test)]
include!("mod_test_imports.rs");
