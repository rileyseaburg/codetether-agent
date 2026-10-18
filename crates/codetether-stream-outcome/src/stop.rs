//! Reasons a stream ended and eligibility for a fresh request.

/// Reason a provider stream stopped.
/// # Examples
/// ```
/// use codetether_stream_outcome::StreamStop;
/// let complete = match StreamStop::Clean { StreamStop::Clean => true, StreamStop::ColdStall | StreamStop::MidStreamStall | StreamStop::PrematureEnd | StreamStop::Fault { .. } => false };
/// assert!(complete);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamStop {
    /// Provider signalled normal end: a `Done` chunk was observed.
    Clean,
    /// Idle timeout before any chunk arrived — nothing committed; retryable.
    ColdStall,
    /// Idle timeout after partial content — partial discarded and retried.
    MidStreamStall,
    /// Byte stream ended (`None`) *before* any `Done` chunk. The HTTP body
    /// closed mid-turn; the model never signalled completion. Restart-eligible
    /// even if partial content was committed, because re-requesting a fresh
    /// stream yields one complete answer (LLM streams are not token-resumable,
    /// so the partial is discarded rather than stitched).
    PrematureEnd,
    /// Terminal error chunk; retryable only if `transient`. Carries the
    /// provider's error message so the final failure surfaces the cause
    /// (e.g. an OpenAI 400 "model is not supported" body) instead of an
    /// opaque "stream faulted" line.
    Fault { transient: bool, message: String },
}

impl StreamStop {
    /// Whether re-opening a fresh stream for the same request is sound.
    pub fn restart_eligible(&self) -> bool {
        matches!(
            self,
            StreamStop::ColdStall
                | StreamStop::MidStreamStall
                | StreamStop::PrematureEnd
                | StreamStop::Fault {
                    transient: true,
                    ..
                }
        )
    }

    /// Whether a retryable incomplete response overrides partial content.
    pub fn restart_over_committed(&self) -> bool {
        matches!(
            self,
            StreamStop::MidStreamStall
                | StreamStop::PrematureEnd
                | StreamStop::Fault {
                    transient: true,
                    ..
                }
        )
    }
}
