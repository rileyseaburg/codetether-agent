//! Bounded native loading with archive-import fallback.
//!
//! Implement [`Host`] for session storage and call [`load`]. [`LoadedSession`]
//! carries the session and retained-window metadata to the interactive client.
mod host;
mod outcome;
mod service;
pub use host::Host;
pub use outcome::LoadedSession;
pub use service::load;
