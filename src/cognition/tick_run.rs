//! One full cognition tick.

use chrono::Utc;

use super::loop_ctx::LoopCtx;
use super::tick_flush::{flush, persist, store_proposals};
use super::tick_govern::{govern, reap_idle};
use super::tick_process::{TickOutput, process_work};
use super::{ThoughtPhase, tick_budget};

/// Run one tick: select work, think, resolve governance, and persist.
pub(super) async fn run_tick(ctx: &LoopCtx) {
    let now = Utc::now();
    *ctx.last_tick_at.write().await = Some(now);

    let mut out = TickOutput::default();
    let work_items = {
        let mut personas = ctx.personas.write().await;
        tick_budget::select_work(&mut personas, now, &mut out.events)
    };
    let active = work_items.len();
    for work in &work_items {
        process_work(ctx, work, active, now, &mut out).await;
    }

    govern(ctx, now, &mut out).await;
    store_proposals(ctx, &mut out).await;
    flush(ctx, out).await;
    reap_idle(ctx, now).await;

    if work_items.iter().any(|w| w.phase == ThoughtPhase::Compress) {
        persist(ctx).await;
    }
}
