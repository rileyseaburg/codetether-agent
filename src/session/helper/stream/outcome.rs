//! Typed stream-stop classification for the Stream Restart Protocol (SRP).
//!
//! The drain loop reports *why* a provider stream stopped so the restart
//! engine ([`super::super::restart`]) can decide whether re-opening a fresh
//! stream is sound. LLM streams are not resumable mid-flight, so only
//! [`StreamStop::ColdStall`] and [`StreamStop::Fault`] are restart-eligible:
//! both mean no usable assistant content was committed and a clean re-request
//! is safe.

use crate::provider::CompletionResponse;

/// Reason a provider stream stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StreamStop {
    /// Provider signalled normal end (`Done` or `None` after content).
    Clean,
    /// Idle timeout before any chunk arrived — nothing committed; retryable.
    ColdStall,
    /// Idle timeout after partial content — partial returned, not retried.
    MidStreamStall,
    /// Terminal error chunk; retryable only if `transient`.
    Fault { transient: bool },
}

impl StreamStop {
    /// Whether re-opening a fresh stream for the same request is sound.
    pub(crate) fn restart_eligible(&self) -> bool {
        matches!(
            self,
            StreamStop::ColdStall | StreamStop::Fault { transient: true }
        )
    }
}

/// Result of one drain pass: the assembled (possibly partial) response and
/// the classified stop reason.
pub(crate) struct DrainOutcome {
    pub(crate) response: Option<CompletionResponse>,
    pub(crate) stop: StreamStop,
}

impl DrainOutcome {
    /// Whether usable assistant content was committed this pass.
    pub(crate) fn committed(&self) -> bool {
        self.response
            .as_ref()
            .is_some_and(|r| !r.message.content.is_empty())
    }
}
