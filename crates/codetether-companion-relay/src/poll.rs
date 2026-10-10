//! Device command poll with stale owner-request expiry.
use crate::{events::event, runtime::Runtime};
use codetether_companion_protocol::{DeviceCommand, DeviceReply, EventKind};

const STALE: &str = "Windows did not provide a fresh screenshot within 60 seconds.";

impl Runtime {
    /// Expire stale requests, even while the device is disconnected.
    pub(crate) fn command(&mut self, now: i64) -> DeviceCommand {
        let stale = self
            .pending
            .as_ref()
            .is_some_and(|p| now - p.created >= 60_000);
        if stale {
            self.pending = None;
            self.status = "error".into();
            self.publish(event(EventKind::Error, Some(STALE.into()), Some("error")));
        }
        self.expire_reply(now);
        DeviceCommand {
            request_id: self.pending.as_ref().map(|p| p.id.clone()),
            reply: self.reply.as_ref().map(|r| DeviceReply {
                id: r.id.clone(),
                text: r.text.clone(),
            }),
        }
    }
}
