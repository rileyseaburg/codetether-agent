//! Completed output-item retention across whole-request stream retries.

#[path = "checkpoint/absorb.rs"]
mod absorb;
#[path = "checkpoint/error.rs"]
mod error;
#[path = "checkpoint/prepend.rs"]
mod prepend;
#[path = "checkpoint/response.rs"]
mod response;

use codetether_provider_core::{CompletionRequest, CompletionResponse, ContentPart};

#[derive(Default)]
pub(super) struct State {
    pub(super) content: Vec<ContentPart>,
}

impl State {
    pub(super) fn absorb(
        &mut self,
        outcome: &codetether_stream_outcome::DrainOutcome,
        request: &mut CompletionRequest,
    ) -> Option<CompletionResponse> {
        absorb::run(self, outcome, request)
    }

    pub(super) fn accept(
        self,
        outcome: codetether_stream_outcome::DrainOutcome,
        retries: u32,
    ) -> anyhow::Result<CompletionResponse> {
        if self.content.is_empty() {
            return codetether_stream_outcome::accept(outcome, retries);
        }
        if response::is_clean_empty(&outcome) {
            return Ok(response::build(self.content, response::usage(&outcome)));
        }
        match codetether_stream_outcome::accept(outcome, retries) {
            Ok(accepted) => Ok(prepend::run(accepted, self.content)),
            Err(error) => Err(self.wrap(error)),
        }
    }

    pub(super) fn wrap(self, source: anyhow::Error) -> anyhow::Error {
        error::wrap(source, self.content)
    }
}

/// Retrieve completed provider items retained in a terminal restart error.
/// # Arguments
/// Receives the error returned by the restart engine.
/// # Returns
/// A cloned checkpoint, or None when no completed items were retained.
/// # Examples
/// ```
/// assert!(codetether_stream_restart::checkpointed_content(&anyhow::anyhow!("plain")).is_none());
/// ```
pub fn content_from_error(error: &anyhow::Error) -> Option<Vec<ContentPart>> {
    error::content(error)
}
