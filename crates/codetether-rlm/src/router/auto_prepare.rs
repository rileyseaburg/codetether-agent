//! Prepare trace state, tool selection, and conversation for analysis.

mod conversation;
mod state;
use super::{host::RouterHost, types::CrateAutoProcessContext};
use crate::{chunker::RlmChunker, config::RlmConfig, context_trace::ContextTrace};
pub(super) use state::Prepared;
use uuid::Uuid;

/// Set up trace, tools, and initial messages.
pub fn prepare(
    output: &str,
    ctx: &CrateAutoProcessContext<'_>,
    _config: &RlmConfig,
    host: &dyn RouterHost,
) -> Prepared {
    let input_tokens = RlmChunker::estimate_tokens(output);
    let trace_id = ctx.trace_id.unwrap_or_else(Uuid::new_v4);
    let trace_budget = input_tokens.saturating_mul(2).max(4096);
    let mut trace = ContextTrace::new(trace_budget);

    let summary_mode = matches!(
        ctx.tool_id,
        "session_context" | "context_reset" | "summary_index"
    );
    let tools = if summary_mode {
        vec![]
    } else {
        host.tool_definitions()
    };

    let conversation = conversation::build(output, ctx, input_tokens, &mut trace);

    Prepared {
        trace_id,
        trace,
        tools,
        conversation,
        summary_mode,
        input_tokens,
    }
}
