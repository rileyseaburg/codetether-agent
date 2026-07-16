//! One deadline-bound provider request for a sub-agent.

use super::state::State;
use crate::provider::{CompletionRequest, CompletionResponse};
use anyhow::Result;
use std::time::{Duration, Instant};
use tokio::time::timeout;

pub(super) enum Outcome {
    Response(CompletionResponse),
    TimedOut,
}

pub(super) async fn execute(state: &State) -> Result<Outcome> {
    let request = CompletionRequest {
        messages: state.messages.clone(),
        tools: state.tools.clone(),
        model: state.model.clone(),
        temperature: state.temperature,
        top_p: None,
        max_tokens: Some(8192),
        stop: Vec::new(),
    };
    let remaining = remaining(state.deadline, Instant::now());
    match timeout(remaining, state.provider.complete(request)).await {
        Ok(response) => Ok(Outcome::Response(response?)),
        Err(_) => {
            tracing::warn!(
                timeout_secs = state.timeout_secs,
                "Sub-agent provider request timed out"
            );
            Ok(Outcome::TimedOut)
        }
    }
}

fn remaining(deadline: Instant, now: Instant) -> Duration {
    deadline.saturating_duration_since(now)
}

#[cfg(test)]
#[path = "request_tests.rs"]
mod tests;
