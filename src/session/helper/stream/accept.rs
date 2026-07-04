//! Final acceptance gate for an SRP drain outcome.
//!
//! Converts a terminal [`DrainOutcome`] into a `Result`. A response with
//! committed content is accepted regardless of stop reason (partials from a
//! mid-stream stall are useful to the agent loop). An empty response is an
//! error whose message reflects whether a retry upstream is warranted.

use anyhow::Result;

use crate::provider::CompletionResponse;

use super::idle_timeout::IDLE_TIMEOUT;
use super::outcome::{DrainOutcome, StreamStop};

/// Accept the final outcome after restarts are exhausted (`attempts` used).
pub(super) fn accept(outcome: DrainOutcome, attempts: u32) -> Result<CompletionResponse> {
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
        StreamStop::PrematureEnd => anyhow::bail!(
            "provider stream ended before completion over {} attempt(s); connection dropped mid-turn",
            attempts + 1
        ),
        StreamStop::Fault { transient } => anyhow::bail!(
            "stream faulted (transient={transient}) with no content over {} attempt(s)",
            attempts + 1
        ),
        _ => anyhow::bail!("provider stream ended without assistant content; none emitted"),
    }
}
