//! Provider HTTP retry logic.
//!
//! Wraps outbound API calls (Z.AI, OpenAI, etc.) with infinite-retry
//! exponential backoff so transient overload / rate-limit / 5xx errors
//! never terminate an agentic session. Used by provider `complete` and
//! `complete_stream` implementations in [`super::zai`].
mod classify;
mod send;
mod stream;

pub use send::send_with_retry;
pub use stream::send_response_with_retry;
