//! Review request construction from a [ReviewSubject] and standing [SYSTEM] instructions.
//!
//! Populate [ReviewSubject], then call [user] to render the reviewer's user turn.
mod subject;
mod system;
mod user;
pub use subject::ReviewSubject;
pub use system::SYSTEM;
pub use user::user;
