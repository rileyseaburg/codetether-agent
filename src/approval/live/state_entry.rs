//! One pending live approval and its active session event channel.

use super::LiveApprovalDecision;
use crate::session::SessionEvent;
use tokio::sync::{mpsc, oneshot};

pub(crate) struct Pending {
    decision: oneshot::Sender<LiveApprovalDecision>,
    events: mpsc::Sender<SessionEvent>,
    tool_call_id: String,
    tool: String,
}

impl Pending {
    pub(crate) fn new(
        decision: oneshot::Sender<LiveApprovalDecision>,
        events: mpsc::Sender<SessionEvent>,
        tool_call_id: String,
        tool: String,
    ) -> Self {
        Self {
            decision,
            events,
            tool_call_id,
            tool,
        }
    }

    pub(super) fn finish(self, id: &str, decision: LiveApprovalDecision) -> bool {
        super::super::decision_event::emit(&self.events, &self.tool_call_id, &self.tool, id);
        self.decision.send(decision).is_ok()
    }
}
