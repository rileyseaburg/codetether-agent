//! Rate-limit-specific retry control and asynchronous backoff.
//!
//! retry accepts a host operation and preserves the original error matching,
//! bounded attempts, and delay calculation without knowing about providers.
mod backoff;
mod run;
pub use run::retry;
