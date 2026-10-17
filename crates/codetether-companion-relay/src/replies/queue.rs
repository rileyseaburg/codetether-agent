//! Validate and queue the owner's text for one device acknowledgement.
use crate::{
    ApiError,
    runtime::{QueuedReply, Runtime},
};
use codetether_companion_protocol::ReplyReceipt;
use serde_json::Value;
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
}
