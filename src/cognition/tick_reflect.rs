//! Belief extraction and merge for the Reflect phase.

use super::loop_ctx::LoopCtx;
use super::tick_process::TickOutput;
use super::{ThoughtWorkItem, belief_merge, beliefs, tick_progress};

/// Extract beliefs from a Reflect-phase thought and merge them into the store.
pub(super) async fn reflect(
    ctx: &LoopCtx,
    work: &ThoughtWorkItem,
    thinking: &str,
    out: &mut TickOutput,
) {
    let extracted =
        beliefs::extract_beliefs_from_thought(ctx.thinker.as_deref(), &work.persona_id, thinking)
            .await;
    if extracted.is_empty() {
        return;
    }
    let created = {
        let mut store = ctx.beliefs.write().await;
        let mut queue = ctx.attention_queue.write().await;
        let events = belief_merge::merge_beliefs(&mut store, &mut queue, work, extracted);
        let created = !events.is_empty();
        out.events.extend(events);
        created
    };
    if created {
        tick_progress::mark_progress(&ctx.personas, &work.persona_id).await;
    }
}
