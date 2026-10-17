use codetether_approval_core::{ApprovalStore, interaction::ExecPolicyAmendment};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use serde_json::json;

pub fn attach_request(
    result: ToolResult,
    tool_name: &str,
    action: &str,
    resource: &str,
    amendment: Option<&ExecPolicyAmendment>,
    reason: &str,
) -> ToolResult {
    match ApprovalStore::open_default()
        .and_then(|store| store.create_request(tool_name, action, resource, reason))
    {
        Ok(request) => {
            if let Some(prefix) = amendment.and_then(ExecPolicyAmendment::prefix_string) {
                codetether_approval_core::session_command_grants::remember_request(
                    &request.id,
                    vec![prefix],
                );
            }
            let result =
                super::approval_output::with_request(result, &request.id, action, resource)
                    .with_metadata("approval_request_id", json!(request.id))
                    .with_metadata("approval_action", json!(action))
                    .with_metadata("approval_resource", json!(resource));
            match amendment {
                Some(value) => result.with_metadata("proposed_execpolicy_amendment", json!(value)),
                None => result,
            }
        }
        Err(error) => result.with_metadata("approval_request_error", json!(error.to_string())),
    }
}

pub fn verified(args: &Value, tool_name: &str, action: &str, resource: &str) -> bool {
    let Some(approval_id) = approval_id(args) else {
        return false;
    };
    ApprovalStore::open_default()
        .and_then(|store| store.verify(approval_id, tool_name, action, resource))
        .is_ok()
}

fn approval_id(args: &Value) -> Option<&str> {
    args.get("approval_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}
