use std::path::Path;
use tokio::sync::mpsc;

use crate::approval::LiveApprovalDecision;
use crate::session::SessionEvent;

use super::types::ApprovalGate;

pub(in crate::session::helper) async fn gate(
    workspace: &Path,
    event_tx: &mpsc::Sender<SessionEvent>,
    tool_call_id: &str,
    tool_name: &str,
    args: serde_json::Value,
) -> ApprovalGate {
    let (blocked, warnings) = super::preflight::inspect(workspace, tool_name, &args).await;
    if let Some(result) = blocked {
        return ApprovalGate::Blocked(args, super::result::tuple(result), warnings);
    }
    let Some(result) = crate::runtime_policy::evaluate_tool_invocation(tool_name, &args).await
    else {
        return ApprovalGate::Ready(args, warnings);
    };
    let Some(request) = super::request::from_result(&result, tool_call_id, tool_name) else {
        return ApprovalGate::Blocked(args, super::result::tuple(result), warnings);
    };
    let id = request.approval_id.clone();
    match crate::approval::live::request(event_tx, request).await {
        LiveApprovalDecision::Approved => {
            ApprovalGate::Ready(super::args::with_approval(args, &id), warnings)
        }
        LiveApprovalDecision::Revised {
            arguments,
            approval_id,
        } => ApprovalGate::Ready(
            super::args::revised(args, arguments, &approval_id),
            warnings,
        ),
        LiveApprovalDecision::Denied { reason } => ApprovalGate::Blocked(
            args,
            super::result::denied(tool_name, &id, reason.as_deref()),
            warnings,
        ),
    }
}
