//! Temporary provider fallback planning and status projection.
//!
//! prepare adds Codex restoration metadata; after_fallback turns it into a
//! scheduled restore request. status describes either stage for the caller.
mod prepare;
mod restore;
mod status;
pub use prepare::prepare;
pub use restore::after_fallback;
pub use status::status;
