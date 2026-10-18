//! Persisted point-in-time copies for detached background conversations.
//!
//! [create] copies the configured transcript tail and inherited metadata into
//! a fresh identity. Implement [Session] for native field and bus access.
mod host;
mod snapshot;
pub use host::Session;
pub use snapshot::{create, tail};
