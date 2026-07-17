//! Final acceptance gate for an SRP drain outcome.
//!
//! Converts a terminal [`DrainOutcome`] into a `Result`. Incomplete transport
//! responses are always rejected so truncated output cannot be committed.

use anyhow::Result;

use crate::provider::CompletionResponse;

use super::idle_timeout::IDLE_TIMEOUT;
use super::outcome::{DrainOutcome, StreamStop};

/// Accept the final outcome after restarts are exhausted (`attempts` used).
pub(super) fn accept(outcome: DrainOutcome, attempts: u32) -> Result<CompletionResponse> {
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
        StreamStop::Fault { transient, message } => anyhow::bail!(
            "stream faulted (transient={transient}) with no content over {} attempt(s): {message}",
            attempts + 1
        ),
        _ => anyhow::bail!("provider stream ended without assistant content; none emitted"),
    }
}
