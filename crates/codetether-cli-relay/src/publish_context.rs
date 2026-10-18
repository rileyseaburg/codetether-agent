//! Merge and publish each specialist context delta.
use crate::{Host, state::Runtime};
use codetether_autochat_core::shared_context::{ContextDelta, publish_context_delta};
impl<H: Host> Runtime<H> {
    pub(super) fn publish_context(
        &mut self,
        context_delta: ContextDelta,
        used_context_rlm: bool,
        to: &str,
        round: usize,
    ) {
        if used_context_rlm {
            self.rlm_context_count += 1;
        }
        self.shared_context.merge_delta(&context_delta);
        let publisher = self.bus.handle(to.to_string());
        publish_context_delta(
            &publisher,
            self.relay.relay_id(),
            &to,
            round,
            self.turns,
            &context_delta,
        );
    }
}
