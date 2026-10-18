//! Shared streaming session events and their structured payloads.
//!
//! [SessionEvent] is generic over the native session snapshot type. Payloads
//! cover compaction, token accounting, RLM progress and stream retries.
//! [SessionEvent::is_durable] classifies records for persistence sinks.
mod compaction_failure;
mod compaction_outcome;
mod compaction_start;
mod durable;
mod event;
mod result;
mod strategy;
mod stream_retry;
mod token_delta;
mod token_estimate;
mod token_source;
mod truncation;
pub use codetether_rlm::events::{RlmCompletion, RlmOutcome, RlmProgressEvent, RlmSubcallFallback};
pub use compaction_failure::CompactionFailure;
pub use compaction_outcome::CompactionOutcome;
pub use compaction_start::CompactionStart;
pub use event::SessionEvent;
pub use result::SessionResult;
pub use strategy::FallbackStrategy;
pub use stream_retry::StreamRetryEvent;
pub use token_delta::TokenDelta;
pub use token_estimate::TokenEstimate;
pub use token_source::TokenSource;
pub use truncation::ContextTruncation;

/// Bounded queued-event delivery with cumulative-text coalescing.
pub mod drain;
