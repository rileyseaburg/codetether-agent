//! Completed output-item retention across whole-request stream retries.

#[path = "checkpoint/absorb.rs"]
mod absorb;
#[path = "checkpoint/error.rs"]
mod error;
#[path = "checkpoint/response.rs"]
mod response;

use crate::provider::{CompletionRequest, CompletionResponse, ContentPart};

#[derive(Default)]
pub(super) struct State {
    pub(super) content: Vec<ContentPart>,
}

impl State {
    pub(super) fn absorb(
        &mut self,
        outcome: &super::super::outcome::DrainOutcome,
        request: &mut CompletionRequest,
    ) -> Option<CompletionResponse> {
        absorb::run(self, outcome, request)
    }

    pub(super) fn accept(
        self,
        outcome: super::super::outcome::DrainOutcome,
        retries: u32,
    ) -> anyhow::Result<CompletionResponse> {
        if self.content.is_empty() {
            return super::super::accept::accept(outcome, retries);
        }
        if response::is_clean_empty(&outcome) {
            return Ok(response::build(self.content, response::usage(&outcome)));
        }
        match super::super::accept::accept(outcome, retries) {
            Ok(accepted) => Ok(response::prepend(accepted, self.content)),
            Err(error) => Err(self.wrap(error)),
        }
    }

    pub(super) fn wrap(self, source: anyhow::Error) -> anyhow::Error {
        error::wrap(source, self.content)
    }
}

pub(in crate::session::helper) fn content_from_error(
    error: &anyhow::Error,
) -> Option<Vec<ContentPart>> {
    error::content(error)
}
