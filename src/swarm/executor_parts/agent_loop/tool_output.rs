//! Context-safe rendering of a sub-agent tool result.

use super::state::State;

pub(super) async fn process(
    state: &State,
    name: &str,
    output: &str,
    success: bool,
) -> (String, bool) {
    codetether_swarm_output::render(
        codetether_swarm_output::Context {
            provider: &state.inner.provider,
            model: &state.inner.model,
            deadline: state.inner.conversation.deadline,
            working_dir: state.inner.working_dir.as_deref(),
        },
        name,
        output,
        success,
    )
    .await
}
