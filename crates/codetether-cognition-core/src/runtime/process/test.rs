use super::Host;
use crate::persona::progress as tick_progress;
use crate::{ThoughtResult, ThoughtWorkItem, runtime::LoopContext, thought::TickOutput};
/// Report the check result, then run any tools the thought requested.
pub(super) async fn test<T, R, H: Host<T, R>>(
    ctx: &LoopContext<T, R>,
    work: &ThoughtWorkItem,
    thought: &ThoughtResult,
    model_backed: bool,
    out: &mut TickOutput,
    host: &H,
) {
    out.events
        .push(super::check::check_result_event(work, thought));
    tick_progress::mark_progress(&ctx.personas, &work.persona_id).await;

    if let Some(tools) = ctx.tools.as_ref().filter(|_| model_backed) {
        let events = host.run_tools(ctx, tools, work, &thought.thinking).await;
        out.events.extend(events);
    }
}
