//! Bounded HTTP retry helpers with exponential backoff.
//!
//! Use [`send_with_retry`] for buffered responses and
//! [`send_response_with_retry`] before consuming a streaming response.
//! Both permit five attempts; permanent failures and exhausted budgets return
//! errors. Successful response content is never classified as an error.
//! [`timing`] exposes jittered backoff and provider-suggested retry delays.
//!
//! # Examples
//!
//! ```rust
//! use codetether_provider_core::retry::timing::from_message;
//! assert_eq!(from_message("try again in 2s"), Some(std::time::Duration::from_secs(2)));
//! ```

mod classify;
mod delay;
mod network;
mod send;
mod send_body;
mod stream;
mod stream_body;
pub mod timing;

pub use send::send_with_retry;
pub use stream::send_response_with_retry;

const MAX_ATTEMPTS: u32 = 5;

#[cfg(test)]
mod classify_tests;
#[cfg(test)]
mod send_tests;
#[cfg(test)]
mod stream_tests;
