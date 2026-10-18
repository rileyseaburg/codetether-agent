//! Decide whether this turn warrants another specialist.
use crate::{Host, state::Runtime};
use codetether_autochat_core::{
    AUTOCHAT_MAX_AGENTS, AUTOCHAT_MAX_DYNAMIC_SPAWNS, AUTOCHAT_SPAWN_CHECK_MIN_CHARS,
};
use codetether_relay_planner::decide_dynamic_spawn_with_registry;
impl<H: Host> Runtime<H> {
    pub(super) async fn spawn(
        &mut self,
        task: &str,
        model_ref: &str,
        output: &str,
        round: usize,
        idx: usize,
    ) {
        let can_attempt_spawn = self.dynamic_spawn_count < AUTOCHAT_MAX_DYNAMIC_SPAWNS
            && self.team.ordered_agents.len() < AUTOCHAT_MAX_AGENTS
            && output.len() >= AUTOCHAT_SPAWN_CHECK_MIN_CHARS;

        if can_attempt_spawn
            && let Some(resolved_registry) = &self.registry
            && let Some((profile, reason)) = decide_dynamic_spawn_with_registry(
                task,
                model_ref,
                &output,
                round,
                &self.team.ordered_agents,
                resolved_registry,
            )
            .await
        {
            self.accept_spawn(profile, reason, model_ref, idx).await;
        }
    }
}
