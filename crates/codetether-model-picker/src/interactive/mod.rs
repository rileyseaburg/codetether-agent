//! Interactive model-picker opening, refresh and selection application.
//!
//! Implement [`View`] and [`Session`] for the native client. Session-bound and
//! session-free opening retain their distinct target and unavailable-registry behavior.
mod close;
mod host;
mod open;
mod refresh;
pub use close::{apply, close};
pub use host::{Session, View};
pub use open::{open_for_session, open_without_session};
