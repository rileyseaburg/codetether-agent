//! Final acceptance gate for an SRP drain outcome.
//!
//! Converts a terminal [`DrainOutcome`] into a `Result`. Incomplete transport
//! responses are always rejected so truncated output cannot be committed.

use anyhow::Result;

use codetether_provider_core::CompletionResponse;

use crate::IDLE_TIMEOUT;
use crate::{DrainOutcome, StreamStop};

/// Accept the final outcome after restarts are exhausted (`attempts` used).
/// # Arguments
/// Receives the terminal drain outcome and number of retries already used.
/// # Returns
/// A nonempty response when its stop reason permits acceptance.
/// # Errors
/// Rejects faults, incomplete transport stops and absent assistant content.
/// # Examples
/// ```
/// let outcome = codetether_stream_outcome::DrainOutcome { response: None, completed: vec![], stop: codetether_stream_outcome::StreamStop::Clean };
/// assert!(codetether_stream_outcome::accept(outcome, 0).is_err());
/// ```
pub fn accept(outcome: DrainOutcome, attempts: u32) -> Result<CompletionResponse> {
    if let StreamStop::Fault { transient, message } = &outcome.stop {
        anyhow::bail!(
            "stream faulted (transient={transient}) over {} attempt(s): {message}",
            attempts + 1
        );
    }
    if matches!(
        outcome.stop,
        StreamStop::MidStreamStall | StreamStop::PrematureEnd
    ) {
        anyhow::bail!("temporary provider availability issue; retry the request");
    }
    let response = outcome.response.filter(|r| !r.message.content.is_empty());
    if let Some(r) = response {
        return Ok(r);
    }
    match outcome.stop {
        StreamStop::ColdStall => anyhow::bail!(
            "stream idle timeout after {}s with no content over {} attempt(s); retry the request",
            IDLE_TIMEOUT.as_secs(),
            attempts + 1
        ),
        StreamStop::MidStreamStall | StreamStop::PrematureEnd => {
            unreachable!("handled before accepting response")
        }
        StreamStop::Fault { .. } => unreachable!("handled before accepting response"),
        _ => anyhow::bail!("provider stream ended without assistant content; none emitted"),
    }
}

/// Reject a retryable outcome after the configured restart budget is spent.
/// # Arguments
/// Receives the exhausted outcome and retry count.
/// # Returns
/// An error preserving the terminal reason and retry count.
/// # Examples
/// ```
/// let outcome = codetether_stream_outcome::DrainOutcome { response: None, completed: vec![], stop: codetether_stream_outcome::StreamStop::ColdStall };
/// assert!(codetether_stream_outcome::exhausted(outcome, 2).to_string().contains("2 retries"));
/// ```
pub fn exhausted(outcome: DrainOutcome, retries: u32) -> anyhow::Error {
    let reason = match outcome.stop {
        StreamStop::Fault { message, .. } => message,
        other => format!("{other:?}"),
    };
    anyhow::anyhow!("stream retry limit exhausted after {retries} retries: {reason}")
}
