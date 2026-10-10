//! Owner replies the device types; questions never reach the device.
use crate::{
    ApiError,
    events::event,
    runtime::{QueuedReply, Runtime},
};
use codetether_companion_protocol::{EventKind, ReplyReceipt, TypedAck};
use serde_json::Value;

const STALE_REPLY: &str = "Windows did not type the reply within 60 seconds.";

impl Runtime {
    /// Queue one owner reply for the device to type (1–2000 characters).
    pub(crate) fn queue_reply(
        &mut self,
        value: &Value,
        paired: bool,
        now: i64,
    ) -> Result<ReplyReceipt, ApiError> {
        self.expire_reply(now);
        let text = value
            .get("text")
            .and_then(Value::as_str)
            .filter(|t| !t.trim().is_empty() && t.encode_utf16().count() <= 2000)
            .ok_or_else(|| ApiError::new(400, "Reply must contain 1–2000 characters"))?;
        if !paired || self.stopped || self.status == "paused" || self.reply.is_some() {
            return Err(ApiError::new(
                409,
                "Device unpaired or a reply is already queued",
            ));
        }
        let id = uuid::Uuid::new_v4().to_string();
        self.reply = Some(QueuedReply {
            id: id.clone(),
            text: text.into(),
            created: now,
        });
        Ok(ReplyReceipt { reply_id: id })
    }

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
