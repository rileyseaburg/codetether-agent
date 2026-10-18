//! One deadline-bound provider request for a sub-agent.

use super::state::State;
use anyhow::Result;
pub(super) use codetether_swarm_core::agent_loop::RequestOutcome as Outcome;

pub(super) async fn execute(state: &State) -> Result<Outcome> {
    let request = codetether_swarm_core::agent_loop::request::build(
        &state.conversation.messages,
        &state.tools,
        &state.model,
        state.conversation.temperature,
        crate::session::helper::token::session_completion_max_tokens(),
    );
    codetether_swarm_core::agent_loop::request::execute(
        state.conversation.deadline,
        state.timeout_secs,
        super::completion::run(state, request),
    )
    .await
}
