//! Idempotent consumption; a lost acknowledgement must never repeat keystrokes.
use super::{Device, Error, http, validation, wire};
use codetether_companion_protocol::{Typed, TypedAck};
use tokio_util::sync::CancellationToken;

impl Device {
    /// Marks an attempt before input. False means ack-only, even after Pause/resume.
    pub(crate) fn reserve_reply(&self, id: &str) -> Result<bool, Error> {
        let id = uuid::Uuid::parse_str(id).map_err(|_| Error::InvalidInput)?;
        let mut replies = self.replies.lock().map_err(|_| Error::InvalidResponse)?;
        if replies.contains(&id) {
            return Ok(false);
        }
        if replies.len() >= 4096 {
            return Err(Error::InvalidInput);
        }
        replies.insert(id);
        Ok(true)
    }
    /// Acknowledges consumption, NOT successful insertion; the existing wire is coarse.
    pub(crate) async fn ack_reply(
        &self,
        id: &str,
        cancel: &CancellationToken,
    ) -> Result<(), Error> {
        self.live(cancel)?;
        validation::request_id(id)?;
        let body = wire::body(&TypedAck {
            reply_id: id.to_owned(),
        })?;
        let request = self
            .authorize(self.agent.post(self.path("typed")))?
            .header("Content-Type", "application/json")
            .body(body);
        let _: Typed = http::receive(request, 200, cancel).await?;
        // typed:false also means already cleared/expired; never retype either way.
        self.live(cancel)
    }
}
