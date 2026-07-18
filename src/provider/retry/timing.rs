//! Shared retry timing primitives for provider and session recovery layers.

#[path = "timing/backoff.rs"]
mod backoff;
#[path = "timing/retry_after.rs"]
mod retry_after;

pub(crate) use backoff::jittered;
pub(crate) use retry_after::from_message;

#[cfg(test)]
#[path = "timing/backoff_tests.rs"]
mod backoff_tests;
#[cfg(test)]
#[path = "timing/retry_after_tests.rs"]
mod retry_after_tests;
