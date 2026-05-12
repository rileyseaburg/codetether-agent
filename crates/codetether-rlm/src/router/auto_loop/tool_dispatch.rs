//! Structured tool-call dispatch during the RLM loop.

use tracing::info;
use crate::context_trace::{ContextEvent, ContextTrace};
use crate::traits::LlmMessage;

use super::super::host::{HostToolResult, RouterHost};

/// Dispatch structured tool calls from the model response.
///
/// Returns `Some(answer)` if a FINAL call was received.
pub(super) fn try_tool_calls(
    host: &mut dyn RouterHost,
    response: &crate::traits::LlmResponse,
    conversation: &mut Vec<LlmMessage>,
    trace: &mut ContextTrace,
    summary_mode: bool,
) -> Option<String> {
    if summary_mode || response.tool_calls.is_empty() { return None; }
    info!(count = response.tool_calls.len(), "RLM router: dispatching structured tool calls");
    let mut results: Vec<LlmMessage> = Vec::new();
    let mut final_answer: Option<String> = None;

    for tc in &response.tool_calls {
        let args_str = tc.arguments.to_string();
        trace.log_event(ContextEvent::ToolCall {
            name: tc.name.clone(),
            arguments_preview: args_str.chars().take(200).collect(),
            tokens: ContextTrace::estimate_tokens(&args_str),
        });
        match host.dispatch(&tc.name, &args_str) {
            Some(HostToolResult::Final(a)) => {
                results.push(LlmMessage::tool_result(&tc.id, "FINAL received"));
                final_answer = Some(a);
                break;
            }
            Some(HostToolResult::Output(o)) => results.push(LlmMessage::tool_result(&tc.id, &o)),
            None => results.push(LlmMessage::tool_result(&tc.id, &format!("Unknown tool: {}", tc.name))),
        }
    }
    for r in results { conversation.push(r); }
    final_answer
}
