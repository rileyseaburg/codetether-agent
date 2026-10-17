//! Acknowledge or expire a queued device reply and publish the outcome.
use crate::{events::event, runtime::Runtime};
use codetether_companion_protocol::{EventKind, TypedAck};
const STALE_REPLY: &str = "Windows did not type the reply within 60 seconds.";
impl Runtime {
    /// Clear a queued reply after the device typed or refused it; the owner
    /// hears the outcome through the snapshot this publishes.
    pub(crate) fn ack_reply(&mut self, ack: &TypedAck) -> bool {
        let typed = self.reply.as_ref().is_some_and(|r| r.id == ack.reply_id);
        if typed {
            self.reply = None;
            self.publish_state();
        }
        typed
    }
    /// Drop a stale queued reply after 60 s; the owner hears it via this error event.
    pub(crate) fn expire_reply(&mut self, now: i64) {
        if self
            .reply
            .as_ref()
            .is_some_and(|r| now - r.created >= 60_000)
        {
            self.reply = None;
            self.publish(event(EventKind::Error, Some(STALE_REPLY.into()), None));
        }
    }
}
