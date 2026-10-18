use super::{
    Host, compress as tick_compress, propose as tick_propose, reflect as tick_reflect,
    test as tick_test,
};
use crate::{
    ThoughtPhase,
    persona::accounting as tick_charge,
    thought::{build as tick_event, context as context_select},
};
use crate::{ThoughtWorkItem, runtime::LoopContext, thought::TickOutput};
use chrono::{DateTime, Utc};
/// How many prior events are fed back as context.
const CONTEXT_LIMIT: usize = 8;

/// Think one work item and collect everything it produced.
pub async fn process_work<T, R, H: Host<T, R>>(
    ctx: &LoopContext<T, R>,
    work: &ThoughtWorkItem,
    active_persona_count: usize,
    now: DateTime<Utc>,
    out: &mut TickOutput,
    host: &H,
) {
    let context =
        context_select::recent_persona_context(&ctx.events, &work.persona_id, CONTEXT_LIMIT).await;
    let thought = host.generate(ctx.thinker.as_deref(), work, &context).await;
    let model_backed = thought.model.is_some();

    out.events.push(tick_event::thought_event(
        work,
        &thought,
        context.len(),
        Utc::now(),
    ));
    tick_charge::charge_thought(&ctx.personas, &work.persona_id, &thought, model_backed).await;

    match work.phase {
        ThoughtPhase::Observe => {}
        ThoughtPhase::Reflect => {
            if model_backed {
                tick_reflect::reflect(ctx, work, &thought.thinking, out, host).await;
            }
            if tick_propose::is_checkpoint(work.thought_count) {
                tick_propose::propose(ctx, work, &thought, active_persona_count, out).await;
            }
        }
        ThoughtPhase::Test => tick_test::test(ctx, work, &thought, model_backed, out, host).await,
        ThoughtPhase::Compress => {
            tick_compress::compress(ctx, work, &thought, context.len(), now, out).await;
        }
    }
}
