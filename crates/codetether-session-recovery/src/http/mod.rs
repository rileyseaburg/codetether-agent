//! Caller-authorized HTTP recovery of durable session identities.
//!
//! Implement [`HttpState`] for control-plane configuration and
//! [`crate::RecoverySession`] for durable storage, then merge [`router`].
mod handler;
mod routes;
mod state;
pub use routes::router;
pub use state::HttpState;
