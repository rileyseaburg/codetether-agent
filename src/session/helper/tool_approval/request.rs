use crate::approval::{ExecPolicyAmendment, LiveApprovalRequest};
use crate::tool::ToolResult;
use serde_json::Value;

use super::result::text;

pub(super) fn from_result(
    result: &ToolResult,
    tool_call_id: &str,
    tool: &str,
) -> Option<LiveApprovalRequest> {
    let id = text(&result.metadata, "approval_request_id")?;
    let request = LiveApprovalRequest::new(
        id,
        tool_call_id.to_string(),
        tool.to_string(),
        text(&result.metadata, "approval_action").unwrap_or_else(|| "execute".into()),
        text(&result.metadata, "approval_resource").unwrap_or_else(|| tool.into()),
        text(&result.metadata, "policy_reason").unwrap_or_else(|| "runtime policy".into()),
    );
    Some(match amendment(&result.metadata) {
        Some(value) => request.with_execpolicy_amendment(value),
        None => request,
    })
}

fn amendment(map: &std::collections::HashMap<String, Value>) -> Option<ExecPolicyAmendment> {
    serde_json::from_value(map.get("proposed_execpolicy_amendment")?.clone()).ok()
}
