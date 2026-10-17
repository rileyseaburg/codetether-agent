//! Stream tool-call fragments.
use crate::*;
pub(super) fn push(choice: &StreamChoice, chunks: &mut Vec<StreamChunk>) {
    if let Some(tool_calls) = &choice.delta.tool_calls {
        for tc in tool_calls {
            // Log tool call metadata for debugging
            tracing::trace!(
                tool_call_index = tc.index,
                tool_call_id = ?tc.id,
                "Processing StepFun streaming tool call"
            );
            if let Some(id) = &tc.id
                && let Some(func) = &tc.function
                && let Some(name) = &func.name
            {
                chunks.push(StreamChunk::ToolCallStart {
                    id: id.clone(),
                    name: name.clone(),
                });
            }
            if let Some(func) = &tc.function
                && let Some(args) = &func.arguments
                && !args.is_empty()
            {
                chunks.push(StreamChunk::ToolCallDelta {
                    id: tc.id.clone().unwrap_or_default(),
                    arguments_delta: args.clone(),
                });
            }
        }
    }
}
