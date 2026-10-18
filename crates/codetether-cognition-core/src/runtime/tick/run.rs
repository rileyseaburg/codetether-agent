use super::{
    Host,
    governance::{govern, reap_idle},
    publication::{flush, store_proposals},
};
use crate::{ThoughtPhase, runtime::LoopContext, thought::TickOutput};
use chrono::Utc;
/// Run one tick: select work, think, resolve governance, and persist.
pub async fn run_tick<T, R, H: Host<T, R>>(ctx: &LoopContext<T, R>, host: &H) {
    let now = Utc::now();
    *ctx.last_tick_at.write().await = Some(now);

    let mut out = TickOutput::default();
    let work_items = {
        let mut personas = ctx.personas.write().await;
        crate::persona::scheduling::select_work(&mut personas, now, &mut out.events)
    };
    let active = work_items.len();
    for work in &work_items {
        host.process_work(ctx, work, active, now, &mut out).await;
    }

    govern(ctx, now, &mut out).await;
    store_proposals(ctx, &mut out).await;
    flush(ctx, out).await;
    reap_idle(ctx, now).await;

    if work_items.iter().any(|w| w.phase == ThoughtPhase::Compress) {
        host.persist(ctx).await;
    }
}
