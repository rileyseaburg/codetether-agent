//! Receive a correlated handoff and add shared context.
use crate::{Host, state::Runtime};
use codetether_autochat_core::{
    shared_context::{compose_prompt_with_context, drain_context_updates},
    transport::consume_handoff_by_correlation,
};
impl<H: Host> Runtime<H> {
    pub(super) async fn receive(&mut self, to: &str, from: &str) -> Option<String> {
        self.turns += 1;
        let _ = drain_context_updates(
            &mut self.context_receiver,
            self.relay.relay_id(),
            &mut self.shared_context,
        );
        let correlation_id = self.relay.send_handoff(&from, &to, &self.baton);
        let consumed_handoff = match consume_handoff_by_correlation(
            &mut self.team.relay_receivers,
            to,
            &correlation_id,
        )
        .await
        {
            Ok(handoff) => handoff,
            Err(err) => {
                self.status = "bus_error".to_string();
                self.failure_note = Some(format!(
                    "Failed to consume handoff for @{to} (correlation={correlation_id}): {err}"
                ));
                return None;
            }
        };
        let prompt_input = compose_prompt_with_context(&consumed_handoff, &self.shared_context);

        Some(prompt_input)
    }
}
