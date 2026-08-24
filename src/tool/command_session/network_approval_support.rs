//! Approval helper shared by network sandbox tests.

use crate::approval::ApprovalStore;
use crate::config::Config;
use serde_json::Value;

pub(super) fn grant(args: &Value) -> String {
    let blocked = crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(), "exec_command", args,
    ).expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(request_id, "test", "sandboxed network probe")
        .expect("approve");
    request_id.to_string()
}