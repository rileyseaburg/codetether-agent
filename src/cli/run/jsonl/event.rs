use crate::provider::Usage;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
#[rustfmt::skip]
pub(super) enum RunEvent<'a> {
    #[serde(rename = "run.started")] Started,
    #[serde(rename = "run.completed")] Completed { #[serde(skip_serializing_if = "Option::is_none")] session_id: Option<&'a str>, response: &'a str, #[serde(skip_serializing_if = "Option::is_none")] usage: Option<&'a Usage> },
    #[serde(rename = "run.failed")] Failed { error: &'a str, #[serde(skip_serializing_if = "Option::is_none")] response: Option<&'a str> },
    #[serde(rename = "run.item_started")] ItemStarted { item_id: &'a str, timestamp_ms: u64 },
    #[serde(rename = "run.item_completed")] ItemCompleted { item_id: &'a str, timestamp_ms: u64 },
    #[serde(rename = "run.text_chunk")] TextChunk { item_id: &'a str, timestamp_ms: u64, text: &'a str },
    #[serde(rename = "run.text_complete")] TextComplete { item_id: &'a str, timestamp_ms: u64, text: &'a str },
    #[serde(rename = "run.tool_call_started")] ToolCallStarted { item_id: &'a str, tool_call_id: &'a str, timestamp_ms: u64, name: Option<&'a str>, arguments: Option<&'a str> },
    #[serde(rename = "run.tool_call_completed")] ToolCallCompleted { item_id: &'a str, tool_call_id: &'a str, timestamp_ms: u64, name: Option<&'a str>, success: Option<bool>, duration_ms: Option<u64>, output: Option<&'a str> },
    #[serde(rename = "run.tool_call_metadata")] ToolCallMetadata { item_id: &'a str, tool_call_id: &'a str, timestamp_ms: u64, metadata: &'a serde_json::Value },
    #[serde(rename = "run.command_started")] CommandStarted { item_id: &'a str, tool_call_id: &'a str, timestamp_ms: u64, command: Option<&'a str>, cwd: Option<&'a str> },
    #[serde(rename = "run.command_completed")] CommandCompleted { item_id: &'a str, tool_call_id: &'a str, timestamp_ms: u64, success: bool, duration_ms: u64 },
    #[serde(rename = "run.patch_started")] PatchStarted { patch_id: &'a str, timestamp_ms: u64 },
    #[serde(rename = "run.patch_completed")] PatchCompleted { patch_id: &'a str, timestamp_ms: u64, metadata: &'a serde_json::Value },
    #[serde(rename = "run.patch_approval_required")] PatchApprovalRequired { approval_id: &'a str, patch_id: &'a str, timestamp_ms: u64 },
    #[serde(rename = "run.approval_requested")] ApprovalRequested { approval_id: &'a str, tool_call_id: &'a str, tool: &'a str, action: &'a str, timestamp_ms: u64, proposed_execpolicy_amendment: Option<&'a serde_json::Value>, available_decisions: Option<&'a serde_json::Value> },
    #[serde(rename = "run.approval_decided")] ApprovalDecided { approval_id: &'a str, decision_id: &'a str, status: &'a str, timestamp_ms: u64 },
}
