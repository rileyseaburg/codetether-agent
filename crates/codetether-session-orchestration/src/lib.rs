//! Final-message continuation and monitoring of delegated session work.
//!
//! Implement [Session] for the application's transcript and supply its existing
//! [Journal](codetether_goal_runtime::Journal) resolver. [after_message] drains
//! accepted human input before scheduling an active goal. [monitor] preserves
//! elapsed accounting and durable holds while waiting for delegated cleanup.
mod continuation;
mod drain;
mod monitor;
mod outcome;
mod session;
pub use continuation::{after_message, continue_goal};
pub use drain::drain;
pub use monitor::monitor;
pub use outcome::Continuation;
pub use session::Session;

/// Implementation support for the application-local session binding.
#[doc(hidden)]
pub mod session_binding;
