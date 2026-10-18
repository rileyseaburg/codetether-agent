//! Identity handling for bounded interactive session continuations.
//!
//! Implement [`Session`] for storage state and owner reparenting, then call
//! [`fork_if_truncated`] before displaying a bounded session continuation.
mod host;
mod service;
pub use host::Session;
pub use service::fork_if_truncated;
