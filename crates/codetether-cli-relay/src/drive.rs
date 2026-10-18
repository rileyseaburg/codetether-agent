//! Ordered relay turns and termination checks.
use crate::{Host, state::Runtime};
use codetether_autochat_core::AUTOCHAT_MAX_ROUNDS;
use uuid::Uuid;
impl<H: Host> Runtime<H> {
    pub(super) async fn drive(&mut self, task: &str, model_ref: &str, okr_run_id: Option<Uuid>) {
        'relay_loop: for round in 1..=AUTOCHAT_MAX_ROUNDS {
            let mut idx = 0usize;
            while idx < self.team.ordered_agents.len() {
                let to = self.team.ordered_agents[idx].clone();
                let from = self.source(round, idx);
                let Some(prompt_input) = self.receive(&to, &from).await else {
                    break 'relay_loop;
                };
                let Some(output) = self.prompt(&to, &prompt_input).await else {
                    break 'relay_loop;
                };
                self.record_convergence(&output);
                self.refine(task, model_ref, &to, &output, round).await;
                codetether_relay_okr::progress::advance(
                    &self.kr_targets,
                    &mut self.kr_progress,
                    self.turns,
                    self.team.ordered_agents.len(),
                    AUTOCHAT_MAX_ROUNDS,
                    okr_run_id,
                )
                .await;
                self.spawn(task, model_ref, &output, round, idx).await;
                if self.convergence_hits >= 2 {
                    self.status = "converged".to_string();
                    break 'relay_loop;
                }
                idx += 1;
            }
        }
    }
}
