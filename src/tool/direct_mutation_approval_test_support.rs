//! Shared approval helpers for direct mutation tests.

use crate::approval::ApprovalStore;
use crate::tool::ToolResult;

pub(super) fn id(result: &ToolResult) -> String {
    result.metadata["approval_request_id"]
        .as_str()
        .expect("approval id")
        .to_string()
}

pub(super) fn approve(id: &str) {
    ApprovalStore::open_default().expect("store").approve(id, "test", "allow").expect("approve");
}