//! In-process actor registration and message admission.

use super::DeliveryRecord;
use super::{envelope::ActorEnvelope, id::ActorId, mailbox::ActorMailbox};
use dashmap::DashMap;

/// In-process actor runtime with addressable mailboxes.
///
/// # Examples
/// ```
/// let runtime = codetether_actor::ActorRuntime::new();
/// runtime.register("worker");
/// assert!(runtime.receive(&"worker".into()).is_none());
/// ```
#[derive(Debug, Default)]
pub struct ActorRuntime {
    pub(super) mailboxes: DashMap<ActorId, ActorMailbox>,
    pub(super) records: DashMap<String, DeliveryRecord>,
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
}
