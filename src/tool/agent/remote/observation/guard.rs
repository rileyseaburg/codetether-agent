//! Cancellation-safe completion of remote turn evidence.

use super::finish::record;
use super::types::RemoteTurnGuard;
use crate::tool::agent::message::remote::reply::PeerReply;

impl RemoteTurnGuard {
    /// Records the peer's reply in the transcript and settles the turn.
    pub(in crate::tool::agent) fn settle(mut self, reply: &PeerReply) {
        self.finish(&reply.text, reply.failed);
    }

    fn finish(&mut self, output: &str, failed: bool) {
        record(
            &self.name,
            self.owner_session_id.as_deref(),
            &self.turn_id,
            output,
            failed,
        );
        self.settled = true;
    }
}

impl Drop for RemoteTurnGuard {
    fn drop(&mut self) {
        if !self.settled {
            self.finish("Remote call cancelled before completion", true);
        }
    }
}
