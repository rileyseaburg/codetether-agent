//! Shared application error response policy.
//!
//! [`internal_error`] preserves the server's message-based status classification
//! and returns the original error text for use by HTTP handlers.
mod mapping;
pub use mapping::internal_error;
