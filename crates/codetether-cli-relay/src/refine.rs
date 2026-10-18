//! Refine the next handoff and publish a shared-context delta.
use crate::{Host, state::Runtime};
use codetether_autochat_core::model_rotation::resolve_provider_for_model_autochat;
use codetether_relay_rlm::{
    distill_context_delta_with_rlm, prepare_autochat_handoff_with_registry,
};
impl<H: Host> Runtime<H> {
    pub(super) async fn refine(
        &mut self,
        task: &str,
        model_ref: &str,
        to: &str,
        output: &str,
        round: usize,
    ) {
        let turn_model_ref = self
            .team
            .agent_models
            .get(to)
            .map(String::as_str)
            .unwrap_or(model_ref);
        let (next_handoff, used_rlm) = prepare_autochat_handoff_with_registry(
            task,
            &to,
            &output,
            turn_model_ref,
            self.registry.as_ref(),
        )
        .await;
        if used_rlm {
            self.rlm_handoff_count += 1;
        }
        let turn_context_provider = self
            .registry
            .as_ref()
            .and_then(|r| resolve_provider_for_model_autochat(r.as_ref(), turn_model_ref));
        let (context_delta, used_context_rlm) =
            distill_context_delta_with_rlm(&output, task, &to, turn_context_provider).await;
        self.publish_context(context_delta, used_context_rlm, to, round);
        self.baton = next_handoff;
    }
}
