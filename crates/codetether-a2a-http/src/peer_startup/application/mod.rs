//! Peer process startup bound to application card and router hooks.
//!
//! [`run`] owns the foreground bus, archival setup and shutdown signal.
//! [`background`] prepares a peer on the caller's bus and returns its handle.
mod background;
mod foreground;
pub use background::background;
pub use foreground::run;
