//! Owner fresh-frame requests; the device only sees an opaque ID.
use crate::{
    ApiError,
    runtime::{Pending, Runtime},
};
use codetether_companion_protocol::CaptureRequestReceipt;
use serde_json::Value;

impl Runtime {
    /// Queue an owner question after validating it and the session state.
    pub(crate) fn request(
        &mut self,
        value: &Value,
        paired: bool,
        now: i64,
    ) -> Result<CaptureRequestReceipt, ApiError> {
        self.command(now);
        let question = value
            .get("question")
            .and_then(Value::as_str)
            .filter(|q| !q.trim().is_empty() && q.encode_utf16().count() <= 2000)
            .ok_or_else(|| ApiError::new(400, "Question must contain 1–2000 characters"))?;
        if !paired || self.active.is_some() || self.pending.is_some() {
            return Err(ApiError::new(409, "Device unpaired or capture busy"));
        }
        if self.frames >= 120 {
            return Err(ApiError::new(429, "Session capture budget exhausted"));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let question = question.trim().into();
        self.pending = Some(Pending {
            id: id.clone(),
            question,
            created: now,
        });
        self.status = "requested".into();
        self.publish_state();
        Ok(CaptureRequestReceipt { request_id: id })
    }
}
