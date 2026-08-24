//! Session-grant settlement for parsed approval decisions.

use super::ApprovalDecisionKind;
use crate::approval::ApprovalReceipt;

impl ApprovalDecisionKind {
    /// Activate a session decision or discard approve-once bookkeeping.
    pub fn grant_session(self, receipt: &ApprovalReceipt) {
        let session_receipt =
            matches!(self, Self::ApproveForSession | Self::ApproveWithAmendment).then_some(receipt);
        crate::approval::session_settle::request(&receipt.approval_id, session_receipt);
    }

    /// Discard pending session bookkeeping for a denied request.
    pub fn discard_pending(self, id: &str) {
        crate::approval::session_settle::request(id, None);
    }
}
