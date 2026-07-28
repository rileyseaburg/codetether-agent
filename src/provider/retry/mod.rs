//! Provider HTTP retry logic.
//!
//! Wraps outbound API calls (Z.AI, OpenAI, etc.) with bounded exponential
//! backoff so transient overload / rate-limit / 5xx errors do not terminate
//! an agentic session, while permanent failures (expired plan, insufficient
//! balance, suspended account) and exhausted retry budgets surface as errors
//! instead of looping forever and holding the worker's task slot. Used by
//! provider `complete` and `complete_stream` implementations in
//! [`super::zai`].
mod classify;
#[cfg(test)]
mod classify_tests;
mod send;
#[cfg(test)]
mod send_tests;
mod stream;
pub(crate) mod timing;

pub use send::send_with_retry;
pub use stream::send_response_with_retry;
