//! Interactive new-session creation with preserved storage and view ordering.
//!
//! Implement [`Session`] and [`View`], then call [`create`] to replace the active
//! session while retaining selected preferences and reporting storage failures.
mod create;
mod persistence;
mod preferences;
mod session;
mod view;
pub use create::create;
pub use session::Session;
pub use view::{Preferences, View};
