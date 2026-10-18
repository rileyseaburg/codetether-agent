//! Delivery and acknowledgment transitions for runtime mailboxes.
use super::{ActorEnvelope, ActorId, ActorRuntime, DeliveryState};

impl ActorRuntime {
    /// Receives the next message for an actor and marks it delivered.
    pub fn receive(&self, actor: &ActorId) -> Option<ActorEnvelope> {
        let message = self
            .mailboxes
            .get_mut(actor)
            .and_then(|mut box_ref| box_ref.pop())?;
        self.mark(&message.id, DeliveryState::Delivered);
        Some(message)
    }

    /// Acknowledges successful processing of one message.
    pub fn ack(&self, message_id: &str) -> bool {
        self.mark(message_id, DeliveryState::Acked)
    }

    fn mark(&self, message_id: &str, state: DeliveryState) -> bool {
        self.records
            .get_mut(message_id)
            .map(|mut record| record.mark(state))
            .is_some()
    }
}
