use super::Host;
use crate::{ThoughtWorkItem, runtime::LoopContext, thought::TickOutput};
use crate::{beliefs::update as belief_merge, persona::progress as tick_progress};
/// Extract beliefs from a Reflect-phase thought and merge them into the store.
pub(super) async fn reflect<T, R, H: Host<T, R>>(
    ctx: &LoopContext<T, R>,
    work: &ThoughtWorkItem,
    thinking: &str,
    out: &mut TickOutput,
    host: &H,
) {
    let extracted = host
        .extract(ctx.thinker.as_deref(), &work.persona_id, thinking)
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
