//! Approval request creation and session-scope registration.

use crate::approval::{ApprovalStore, ExecPolicyAmendment};
use crate::tool::ToolResult;
use serde_json::json;

pub(in crate::runtime_policy) fn attach_request(
    result: ToolResult,
    tool_name: &str,
    action: &str,
    resource: &str,
    amendment: Option<&ExecPolicyAmendment>,
    args: Option<&serde_json::Value>,
) -> ToolResult {
    match ApprovalStore::open_default()
        .and_then(|store| store.create_request(tool_name, action, resource, "runtime policy"))
    {
        Ok(request) => attach(result, request, action, resource, amendment, args),
        Err(error) => result.with_metadata("approval_request_error", json!(error.to_string())),
    }
}

fn attach(
    result: ToolResult,
    request: crate::approval::ApprovalRequest,
    action: &str,
    resource: &str,
    amendment: Option<&ExecPolicyAmendment>,
    args: Option<&serde_json::Value>,
) -> ToolResult {
    let session_id = args
        .and_then(|value| value.get("__ct_session_id"))
        .and_then(serde_json::Value::as_str);
    crate::approval::session_grants::remember_request(&request.id, session_id);
    if let Some(prefix) = amendment.and_then(ExecPolicyAmendment::prefix_string) {
        let workspace = args.and_then(super::super::session_command::scope::from_args);
        let workspace = workspace.as_deref();
        crate::approval::session_command_grants::remember_scoped_request_in(
            &request.id,
            vec![prefix],
            session_id,
            workspace,
        );
    }
    let result = super::super::approval_output::with_request(result, &request.id, action, resource)
        .with_metadata("approval_request_id", json!(request.id))
        .with_metadata("approval_action", json!(action))
        .with_metadata("approval_resource", json!(resource));
    match amendment {
        Some(value) => result.with_metadata("proposed_execpolicy_amendment", json!(value)),
        None => result,
    }
}
