//! Tests for watchdog stall detection.
//!
//! Split across submodules to honor the 50-line file limit:
//! * [`shared`] — common builders and the timeout constant.
//! * [`no_first_token`] — the start-clock clause and its suppression.
//! * [`rearm`] — detector re-arming after a notification is cleared.

#[path = "tests/shared.rs"]
mod shared;

#[path = "tests/no_first_token.rs"]
mod no_first_token;

#[path = "tests/rearm.rs"]
mod rearm;
