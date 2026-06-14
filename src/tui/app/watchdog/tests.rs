//! Tests for watchdog stall detection.
//!
//! Split across submodules to honor the 50-line file limit:
//! * [`shared`] — common builders and the timeout constant.
//! * [`no_first_token`] — the start-clock clause and its suppression.
//! * [`inactivity`] — the activity-based clause and budget guards.

#[path = "tests/shared.rs"]
mod shared;

#[path = "tests/no_first_token.rs"]
mod no_first_token;

#[path = "tests/inactivity.rs"]
mod inactivity;
