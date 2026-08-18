//! Test-phase reporting and tool execution.

use super::loop_ctx::LoopCtx;
use super::tick_process::TickOutput;
use super::{ThoughtResult, ThoughtWorkItem, tick_progress, tick_test_phase};

/// Report the check result, then run any tools the thought requested.
pub(super) async fn test(
    ctx: &LoopCtx,
    work: &ThoughtWorkItem,
    thought: &ThoughtResult,
    model_backed: bool,
    out: &mut TickOutput,
) {
    out.events
        .push(tick_test_phase::check_result_event(work, thought));
    tick_progress::mark_progress(&ctx.personas, &work.persona_id).await;

    if let Some(tools) = ctx.tools.as_ref().filter(|_| model_backed) {
        let events = tick_test_phase::run_tools(
            &ctx.personas,
            ctx.thinker.as_deref(),
            tools,
            work,
            &thought.thinking,
        )
        .await;
        out.events.extend(events);
    }
}
