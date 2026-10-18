//! Native session adapter for shared approval sequencing.
use crate::session::SessionEvent;
use codetether_tool_approval::ApprovalGate;
use std::path::Path;
use tokio::sync::mpsc;

pub(in crate::session::helper) async fn gate(
    workspace: &Path,
    event_tx: &mpsc::Sender<SessionEvent>,
    tool_call_id: &str,
    tool_name: &str,
    args: serde_json::Value,
) -> ApprovalGate {
    let host = super::host::NativeHost { event_tx };
    codetether_tool_approval::gate(workspace, &host, tool_call_id, tool_name, args).await
}
