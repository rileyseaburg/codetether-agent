use std::path::Path;
use tokio::sync::mpsc;

use crate::session::SessionEvent;

use super::types::ApprovalGate;

pub(in crate::session::helper) async fn gate(
    workspace: &Path,
    event_tx: &mpsc::Sender<SessionEvent>,
    tool_call_id: &str,
    tool_name: &str,
    args: serde_json::Value,
) -> ApprovalGate {
    let args = match super::review::arguments(event_tx, tool_call_id, tool_name, args).await {
        Ok(args) => args,
        Err(blocked) => return blocked,
    };
    if let Some(result) = super::preflight::blocked(workspace, tool_name, &args).await {
        return ApprovalGate::Blocked(args, super::result::tuple(result));
    }
    ApprovalGate::Ready(args)
}
