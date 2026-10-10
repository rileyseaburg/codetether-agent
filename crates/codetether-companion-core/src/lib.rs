//! Memory-only screen-companion authentication and session lifecycle.
//!
//! [`Registry`] owns bounded, expiring sessions; [`OwnerCredential`] verifies
//! the separate owner bearer token. Caller-supplied times are Unix milliseconds
//! from a trusted server clock, never client timestamps. This library has no
//! HTTP server, capture permission, inference, persistence, or event transport.
//!
//! ```
//! use codetether_companion_core::Registry;
//! use codetether_companion_protocol::SessionInput;
//! let mut sessions = Registry::default();
//! let receipt = sessions.create(SessionInput {
//!     model: "provider/vision".into(), prompt: "Describe".into(), interval_seconds: 30,
//! }, 1_000_000)?;
//! assert_eq!(receipt.interval_seconds, 30);
//! # Ok::<(), codetether_companion_core::Error>(())
//! ```
mod auth;
mod create;
mod error;
mod input;
mod lifecycle;
mod owner;
mod pairing;
mod registry;
mod secrets;
mod session;
pub use error::Error;
pub use owner::{OwnerCredential, require_origin};
pub use registry::Registry;
pub use session::Session;
