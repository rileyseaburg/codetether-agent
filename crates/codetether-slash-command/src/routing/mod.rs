//! TUI command recognition and unknown-message suppression policy.
//!
//! Use [`leading`] before conditional relay/session dispatch, [`session`] for
//! session actions and [`suppress_unknown`] after fallback processing. Names and
//! priority preserve the existing interactive command behavior.
mod leading;
mod session;
mod types;
mod unknown;
pub use leading::leading;
pub use session::session;
pub use types::{LeadingCommand, SessionCommand};
pub use unknown::suppress_unknown;
