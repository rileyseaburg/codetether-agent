//! Shut down specialists, persist progress and assemble the final report.
use crate::{Host, state::Runtime};
use codetether_relay_report::AutochatCliResult;
use uuid::Uuid;
impl<H: Host> Runtime<H> {
    pub(super) async fn finish(
        self,
        model_ref: &str,
        okr_run_id: Option<Uuid>,
    ) -> AutochatCliResult {
        self.relay.shutdown_agents(&self.team.ordered_agents);

        // Update OKR run with final progress if associated
        self.record_final_progress(okr_run_id).await;

        let summary = codetether_relay_report::summary(&codetether_relay_report::Summary {
            status: &self.status,
            relay_id: self.relay.relay_id(),
            agent_count: self.team.ordered_agents.len(),
            turns: self.turns,
            final_handoff: &self.baton,
            failure: self.failure_note.as_deref(),
            planner_used: self.planner_used,
            rlm_handoff_count: self.rlm_handoff_count,
            rlm_context_count: self.rlm_context_count,
            shared_context_items: self.shared_context.item_count(),
            dynamic_spawn_count: self.dynamic_spawn_count,
        });

        AutochatCliResult {
            status: self.status,
            relay_id: self.relay.relay_id().to_string(),
            model: model_ref.to_string(),
            agent_count: self.team.ordered_agents.len(),
            turns: self.turns,
            agents: self.team.ordered_agents,
            final_handoff: self.baton,
            summary,
            failure: self.failure_note,
            shared_context_items: self.shared_context.item_count(),
            rlm_context_count: self.rlm_context_count,
        }
    }
}
