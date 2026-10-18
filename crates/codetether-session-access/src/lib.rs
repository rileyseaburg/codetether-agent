//! Caller-authorized workspace access for durable session recovery.
//!
//! authority verifies the caller with the configured control plane; scope
//! compares canonical workspace paths. http_error redacts storage failures.
pub mod authority;
mod errors;
pub mod scope;
pub use errors::http_error;
#[cfg(test)]
mod tests;
