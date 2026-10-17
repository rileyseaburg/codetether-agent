//! One deadline-bound provider request for a sub-agent.

use super::state::State;
use crate::provider::CompletionRequest;
use anyhow::Result;
pub(super) use codetether_swarm_core::agent_loop::RequestOutcome as Outcome;
use std::time::{Duration, Instant};
use tokio::time::timeout;

pub(super) async fn execute(state: &State) -> Result<Outcome> {
    let request = CompletionRequest {
        messages: state.messages.clone(),
        tools: state.tools.clone(),
        model: state.model.clone(),
        temperature: state.temperature,
        top_p: None,
        max_tokens: Some(crate::session::helper::token::session_completion_max_tokens()),
        stop: Vec::new(),
    };
    let remaining = remaining(state.deadline, Instant::now());
    match timeout(remaining, super::completion::run(state, request)).await {
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
