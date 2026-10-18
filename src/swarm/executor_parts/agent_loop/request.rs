//! One deadline-bound provider request for a sub-agent.

use super::state::State;
use anyhow::Result;
pub(super) use codetether_swarm_core::agent_loop::RequestOutcome as Outcome;

pub(super) async fn execute(state: &State) -> Result<Outcome> {
    let request = codetether_swarm_core::agent_loop::request::build(
        &state.messages,
        &state.tools,
        &state.model,
        state.temperature,
        crate::session::helper::token::session_completion_max_tokens(),
    );
    codetether_swarm_core::agent_loop::request::execute(
        state.deadline,
        state.timeout_secs,
        super::completion::run(state, request),
    )
    .await
}
