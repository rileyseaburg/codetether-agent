use crate::cognition::{
    ThinkerClient, ThoughtWorkItem,
    loop_ctx::LoopCtx,
    tick_flush::persist,
    tick_process::{TickOutput, process_work},
};
use crate::tool::ToolRegistry;
use chrono::{DateTime, Utc};
pub(super) struct Native;
impl codetether_cognition_core::runtime::tick::Host<ThinkerClient, ToolRegistry> for Native {
    async fn process_work(
        &self,
        ctx: &LoopCtx,
        work: &ThoughtWorkItem,
        active: usize,
        now: DateTime<Utc>,
        out: &mut TickOutput,
    ) {
        process_work(ctx, work, active, now, out).await;
    }
    async fn persist(&self, ctx: &LoopCtx) {
        persist(ctx).await;
    }
}
