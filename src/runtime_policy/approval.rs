//! Approval request verification and atomic receipt claiming.

use crate::approval::ApprovalStore;
use serde_json::Value;

#[path = "approval/request.rs"]
mod request;
#[path = "approval/tools.rs"]
mod tools;
pub(super) use request::attach_request;
pub(crate) use tools::self_verifying;

pub(super) fn verified(args: &Value, tool_name: &str, action: &str, resource: &str) -> bool {
    let Some(approval_id) = approval_id(args) else {
        return false;
    };
    let Ok(store) = ApprovalStore::open_default() else {
        return false;
    };
    if self_verifying(tool_name) {
        store
            .verify(approval_id, tool_name, action, resource)
            .is_ok()
    } else {
        store
            .claim(approval_id, tool_name, action, resource, "runtime-policy")
            .is_ok()
    }
}

pub(super) fn inspected(args: &Value, tool_name: &str, action: &str, resource: &str) -> bool {
    let Some(approval_id) = approval_id(args) else {
        return false;
    };
    ApprovalStore::open_default()
        .and_then(|store| store.verify(approval_id, tool_name, action, resource))
        .is_ok()
}

pub(super) fn claim(args: &Value, tool_name: &str, action: &str, resource: &str) -> bool {
    let Some(approval_id) = approval_id(args) else {
        return false;
    };
    ApprovalStore::open_default()
        .and_then(|store| store.claim(approval_id, tool_name, action, resource, "runtime-policy"))
        .is_ok()
}

fn approval_id(args: &Value) -> Option<&str> {
    args.get("approval_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}
