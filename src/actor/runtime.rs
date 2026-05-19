use super::{DeliveryRecord, DeliveryState};
use super::{envelope::ActorEnvelope, id::ActorId, mailbox::ActorMailbox};
use dashmap::DashMap;

/// In-process actor runtime with addressable mailboxes.
#[derive(Debug, Default)]
pub struct ActorRuntime {
    mailboxes: DashMap<ActorId, ActorMailbox>,
    records: DashMap<String, DeliveryRecord>,
}

impl ActorRuntime {
    /// Creates an empty actor runtime.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a mailbox for an actor if one does not exist.
    pub fn register(&self, actor: impl Into<ActorId>) {
        self.mailboxes.entry(actor.into()).or_default();
    }

    /// Sends an envelope to its target actor mailbox.
    pub fn send(&self, message: ActorEnvelope) {
        self.records
            .insert(message.id.clone(), DeliveryRecord::pending());
        self.mailboxes
            .entry(message.to.clone())
            .or_default()
            .push(message);
    }

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
