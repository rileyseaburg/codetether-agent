//! Typed stream-stop classification for the Stream Restart Protocol (SRP).
//!
//! The drain loop reports *why* a provider stream stopped so the restart
//! engine ([`super::super::restart`]) can decide whether re-opening a fresh
//! stream is sound. LLM streams are not resumable mid-flight, so restarts are
//! only sound for [`StreamStop::ColdStall`], [`StreamStop::PrematureEnd`], and
//! transient [`StreamStop::Fault`]: each means a clean re-request yields one
//! complete answer (any partial is discarded, never token-stitched).

use crate::provider::CompletionResponse;

/// Reason a provider stream stopped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StreamStop {
    /// Provider signalled normal end: a `Done` chunk was observed.
    Clean,
    /// Idle timeout before any chunk arrived — nothing committed; retryable.
    ColdStall,
    /// Idle timeout after partial content — partial returned, not retried.
    MidStreamStall,
    /// Byte stream ended (`None`) *before* any `Done` chunk. The HTTP body
    /// closed mid-turn; the model never signalled completion. Restart-eligible
    /// even if partial content was committed, because re-requesting a fresh
    /// stream yields one complete answer (LLM streams are not token-resumable,
    /// so the partial is discarded rather than stitched).
    PrematureEnd,
    /// Terminal error chunk; retryable only if `transient`.
    Fault { transient: bool },
}

impl StreamStop {
    /// Whether re-opening a fresh stream for the same request is sound.
    pub(crate) fn restart_eligible(&self) -> bool {
        matches!(
            self,
            StreamStop::ColdStall | StreamStop::PrematureEnd | StreamStop::Fault { transient: true }
        )
    }

    /// Whether a restart should proceed *even if* partial content was
    /// committed this pass. Only a [`StreamStop::PrematureEnd`] discards the
    /// committed partial to re-request a clean, complete stream; an idle
    /// [`StreamStop::MidStreamStall`] keeps its partial instead.
    pub(crate) fn restart_over_committed(&self) -> bool {
        matches!(self, StreamStop::PrematureEnd)
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
