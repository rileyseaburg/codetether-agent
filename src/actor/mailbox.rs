use super::{envelope::ActorEnvelope, state::DeliveryRecord};
use std::collections::VecDeque;

/// FIFO inbox owned by a single actor.
#[derive(Debug, Default)]
pub struct ActorMailbox {
    queue: VecDeque<ActorEnvelope>,
}

impl ActorMailbox {
    /// Adds a message to the back of the mailbox.
    pub fn push(&mut self, message: ActorEnvelope) {
        self.queue.push_back(message);
    }

    /// Removes the next message from the front of the mailbox.
    pub fn pop(&mut self) -> Option<ActorEnvelope> {
        self.queue.pop_front()
    }

    /// Returns true when no messages are queued.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// Message plus current delivery metadata.
#[derive(Debug, Clone)]
pub struct MailboxDelivery {
    pub message: ActorEnvelope,
    pub record: DeliveryRecord,
}
