//! Cleanup and optional activation of pending session grants.

use super::ApprovalReceipt;

pub(crate) fn request(id: &str, session_receipt: Option<&ApprovalReceipt>) {
    if let Some(receipt) = session_receipt {
        super::session_grants::grant(receipt);
        super::session_command_grants::grant_for_request(id);
    }
    super::session_grants::discard_request(id);
    super::session_command_grants::discard_request(id);
}
