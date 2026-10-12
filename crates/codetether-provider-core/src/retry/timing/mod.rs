//! Retry timing primitives: [`jittered`] backoff and [`from_message`] delays.
//!
//! Use [`jittered`] to desynchronize retries across callers and [`from_message`]
//! to honor provider delay hints without depending on the agent runtime.
//!
//! # Examples
//!
//! ```rust
//! use codetether_provider_core::retry::timing::from_message;
//! assert_eq!(from_message("no delay hint"), None);
//! ```

mod backoff;
mod retry_after;

pub use backoff::jittered;
pub use retry_after::from_message;

#[cfg(test)]
mod backoff_tests;
#[cfg(test)]
mod retry_after_tests;
