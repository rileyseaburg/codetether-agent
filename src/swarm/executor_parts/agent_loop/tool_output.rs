//! Context-safe rendering of a sub-agent tool result.

use super::{
    super::{agent_loop_large_result, large_result},
    state::State,
};
use crate::swarm::token_truncate::truncate_single_result;
use crate::tool::feedback::render_with_dir;
use std::sync::Arc;

pub(super) async fn process(
    state: &State,
    name: &str,
    output: &str,
    success: bool,
) -> (String, bool) {
    let (output, timed_out) = if output.len() > large_result::RLM_THRESHOLD_CHARS {
        agent_loop_large_result::process(
            output,
            name,
            Arc::clone(&state.provider),
            &state.model,
            state.deadline,
        )
        .await
    } else {
        (
            truncate_single_result(output, large_result::SIMPLE_TRUNCATE_CHARS),
            false,
        )
    };
    (
        render_with_dir(name, success, state.working_dir.as_deref(), &output),
        timed_out,
    )
}
