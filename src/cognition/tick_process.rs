//! Per-persona thought processing for one tick.

use chrono::{DateTime, Utc};

use super::loop_ctx::LoopCtx;
use super::{
    MemorySnapshot, Proposal, ThoughtEvent, ThoughtPhase, ThoughtWorkItem, context_select,
    thought_generate, tick_charge, tick_compress, tick_event, tick_propose, tick_reflect,
    tick_test,
};

/// Products of thinking one work item.
#[derive(Default)]
pub(super) struct TickOutput {
    pub events: Vec<ThoughtEvent>,
    pub snapshots: Vec<MemorySnapshot>,
    pub proposals: Vec<Proposal>,
}

/// How many prior events are fed back as context.
const CONTEXT_LIMIT: usize = 8;

/// Think one work item and collect everything it produced.
pub(super) async fn process_work(
    ctx: &LoopCtx,
    work: &ThoughtWorkItem,
    active_persona_count: usize,
    now: DateTime<Utc>,
    out: &mut TickOutput,
) {
    let context =
        context_select::recent_persona_context(&ctx.events, &work.persona_id, CONTEXT_LIMIT).await;
    let thought =
        thought_generate::generate_phase_thought(ctx.thinker.as_deref(), work, &context).await;
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
                tick_reflect::reflect(ctx, work, &thought.thinking, out).await;
            }
            if tick_propose::is_checkpoint(work.thought_count) {
                tick_propose::propose(ctx, work, &thought, active_persona_count, out).await;
            }
        }
        ThoughtPhase::Test => tick_test::test(ctx, work, &thought, model_backed, out).await,
        ThoughtPhase::Compress => {
            tick_compress::compress(ctx, work, &thought, context.len(), now, out).await;
        }
    }
}
