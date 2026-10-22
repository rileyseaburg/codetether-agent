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

/// Owned session completion notices and caught-panic conversion.
pub mod completion;

/// Frontend activity tracking and ordered event delivery.
pub mod pipeline;

/// Ordered frontend lifecycle transitions for session events.
pub mod lifecycle;

/// Dependencies used by application-local event declarations.
#[doc(hidden)]
pub mod __private {
    pub use tokio::sync::mpsc;
}
pub mod activity;
