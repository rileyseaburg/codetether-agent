//! Bind shared phase processing to native inference and tools.
mod host;
use super::{ThoughtWorkItem, loop_ctx::LoopCtx};
use chrono::{DateTime, Utc};
pub(super) use codetether_cognition_core::thought::TickOutput;
pub(super) async fn process_work(
    ctx: &LoopCtx,
    work: &ThoughtWorkItem,
    active_persona_count: usize,
    now: DateTime<Utc>,
    out: &mut TickOutput,
) {
    codetether_cognition_core::runtime::process::process_work(
        ctx,
        work,
        active_persona_count,
        now,
        out,
        &host::Native,
    )
    .await;
}
