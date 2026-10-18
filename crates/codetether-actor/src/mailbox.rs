//! FIFO queues and delivery metadata for actor mailboxes.

use super::{envelope::ActorEnvelope, state::DeliveryRecord};
use std::collections::VecDeque;

/// FIFO inbox owned by a single actor.
///
/// # Examples
/// ```
/// let mailbox = codetether_actor::ActorMailbox::default();
/// assert!(mailbox.is_empty());
/// ```
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
///
/// # Examples
/// ```
/// let delivery = codetether_actor::MailboxDelivery {
///     message: codetether_actor::ActorMessage::new("a", "b", "run", serde_json::json!({})).into_envelope(),
///     record: codetether_actor::DeliveryRecord::pending(),
/// };
/// assert_eq!(delivery.message.to.0, "b");
/// ```
#[derive(Debug, Clone)]
pub struct MailboxDelivery {
    /// Delivered envelope.
    pub message: ActorEnvelope,
    /// Associated delivery state.
    pub record: DeliveryRecord,
}
