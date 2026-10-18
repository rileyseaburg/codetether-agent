//! Final-answer insertion, steering, validation retry and goal continuation.
//!
//! [finish_or_retry] sequences a native [Host] without owning session storage
//! or validation services. Retry state remains on the shared [crate::LoopState].
mod finish;
mod host;
pub use finish::finish_or_retry;
pub use host::Host;
