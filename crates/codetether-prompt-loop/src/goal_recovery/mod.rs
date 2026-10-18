//! Whole-turn recovery policy for active goals after provider failures.
//!
//! [after] groups repeated failures, stops after the existing retry threshold,
//! emits retry notifications and asks the native [Host] to continue the goal.
//! Status persistence and goal lookup stay behind the host boundary.
mod after;
mod host;
mod notify;
mod record;
mod signature;
mod status;
pub use after::after;
pub use host::Host;
const MAX_FAILURE_TURNS: u8 = 3;
