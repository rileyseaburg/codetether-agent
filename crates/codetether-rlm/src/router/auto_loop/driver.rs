//! Bounded orchestration of analysis iterations.
use super::{initial, step, support};

use crate::router::{
    host::RouterHost,
    types::{CrateAutoProcessContext, LoopOutcome},
};

use crate::{
    config::RlmConfig,
    context_trace::ContextTrace,
    traits::{LlmMessage, ToolDefinition},
};

use tracing::warn;

/// Run the iterative analysis loop.
pub async fn run(
    ctx: &CrateAutoProcessContext<'_>,
    config: &RlmConfig,
    host: &mut dyn RouterHost,
    conversation: &mut Vec<LlmMessage>,
    trace: &mut ContextTrace,
    tools: &[ToolDefinition],
    summary_mode: bool,
) -> LoopOutcome {
    let mut state = initial::initial();
    for i in 0..config.max_iterations {
        state.iterations = i + 1;
        trace.next_iteration();
        support::emit_progress(ctx, state.iterations, config.max_iterations);
        if support::check_abort(ctx) {
            state.aborted = true;
            break;
        }
        if !step::run(
            ctx,
            host,
            conversation,
            trace,
            tools,
            summary_mode,
            &mut state,
        )
        .await
        {
            break;
        }
        state.subcalls += 1;
        if state.subcalls >= config.max_subcalls {
            warn!(subcalls = state.subcalls, "RLM: Max subcalls reached");
            break;
        }
    }
    state
}
