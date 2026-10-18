//! Record final objective progress before assembling the result.
use crate::{Host, state::Runtime};
use uuid::Uuid;
impl<H: Host> Runtime<H> {
    pub(super) async fn record_final_progress(&self, okr_run_id: Option<Uuid>) {
        codetether_relay_okr::progress::finish(
            okr_run_id,
            &self.kr_progress,
            codetether_relay_okr::progress::Report {
                relay_id: self.relay.relay_id(),
                turns: self.turns,
                agent_count: self.team.ordered_agents.len(),
                status: &self.status,
                rlm_handoff_count: self.rlm_handoff_count,
                dynamic_spawn_count: self.dynamic_spawn_count,
            },
        )
        .await;
    }
}
