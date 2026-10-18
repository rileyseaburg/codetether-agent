//! One deadline-bound provider request for a sub-agent.

use super::state::State;
use anyhow::Result;
pub(super) use codetether_swarm_core::agent_loop::RequestOutcome as Outcome;

pub(super) async fn execute<H: crate::Host>(state: &State<H>) -> Result<Outcome> {
    let request = codetether_swarm_core::agent_loop::request::build(
        &state.inner.conversation.messages,
        &state.inner.tools,
        &state.inner.model,
        state.inner.conversation.temperature,
        codetether_context_policy::derivation::completion_limit::session_completion_max_tokens(),
    );
    codetether_swarm_core::agent_loop::request::execute(
        state.inner.conversation.deadline,
        state.inner.timeout_secs,
        super::completion::run(state, request),
    )
    .await
}
