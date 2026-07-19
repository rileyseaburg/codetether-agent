//! Cancellation-safe completion of remote turn evidence.

use super::finish::record;
use super::types::RemoteTurnGuard;
use crate::tool::ToolResult;

impl RemoteTurnGuard {
    /// Records a completed remote result and settles the turn.
    pub(in crate::tool::agent) fn complete(mut self, result: &ToolResult) {
        self.finish(&result.output, !result.success);
    }

    /// Records a transport failure and settles the turn.
    pub(in crate::tool::agent) fn fail(mut self, error: &str) {
        self.finish(&format!("Remote call failed: {error}"), true);
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
