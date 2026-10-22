//! Sequence preflight, policy and interactive approval decisions.

use crate::Host;
use std::path::Path;

use codetether_approval_core::interaction::LiveApprovalDecision;

use super::types::ApprovalGate;

/// Resolve diagnostic checks, runtime policy and any required user decision.
///
/// # Arguments
/// `workspace` scopes diagnostics; `host` supplies application services.
/// `tool_call_id`, `tool_name` and `args` identify the proposed invocation.
///
/// # Returns
/// Approved arguments or a blocking result, together with diagnostic warnings.
///
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_tool_approval::Host) {
/// let outcome = codetether_tool_approval::gate(
///     std::path::Path::new("."), host, "call-1", "read", serde_json::json!({})
/// ).await;
/// let (_args, _block, _warnings) = outcome.into_checked_parts();
/// # }
/// ```
pub async fn gate(
    workspace: &Path,
    host: &impl Host,
    tool_call_id: &str,
    tool_name: &str,
    args: serde_json::Value,
) -> ApprovalGate {
    let (blocked, warnings) = host.preflight(workspace, tool_name, &args).await;
    if let Some(result) = blocked {
        return ApprovalGate::Blocked(args, super::result::tuple(result), warnings);
    }
    let Some(result) = host.policy(tool_name, &args).await else {
        return ApprovalGate::Ready(args, warnings);
    };
    let Some(request) = super::request::from_result(&result, tool_call_id, tool_name) else {
        return ApprovalGate::Blocked(args, super::result::tuple(result), warnings);
    };
    let id = request.approval_id.clone();
    match host.request(request).await {
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
        LiveApprovalDecision::Denied { reason } => ApprovalGate::Denied(
            args,
            super::result::denied(tool_name, &id, reason.as_deref()),
            warnings,
            reason.filter(|text| !text.trim().is_empty()),
        ),
    }
}
