//! Receipt review and live decision collection without approval consumption.

use tokio::sync::mpsc;

use crate::approval::LiveApprovalDecision;
use crate::session::SessionEvent;

use super::types::ApprovalGate;

pub(super) async fn arguments(
    event_tx: &mpsc::Sender<SessionEvent>,
    tool_call_id: &str,
    tool_name: &str,
    args: serde_json::Value,
) -> Result<serde_json::Value, ApprovalGate> {
    let Some(result) = crate::runtime_policy::invocation::review::evaluate(tool_name, &args).await
    else {
        return Ok(args);
    };
    let Some(request) = super::request::from_result(&result, tool_call_id, tool_name) else {
        return Err(ApprovalGate::Blocked(args, super::result::tuple(result)));
    };
    let id = request.approval_id.clone();
    match crate::approval::live::request(event_tx, request).await {
        LiveApprovalDecision::Approved => Ok(super::args::with_approval(args, &id)),
        LiveApprovalDecision::Revised {
            arguments,
            approval_id,
        } => Ok(super::args::revised(args, arguments, &approval_id)),
        LiveApprovalDecision::Denied { reason } => Err(ApprovalGate::Blocked(
            args,
            super::result::denied(tool_name, &id, reason.as_deref()),
        )),
    }
}
