//! Incremental tool start and argument events.
use crate::*;
pub(super) fn push(choice: &StreamChoice, text_buf: &mut String, chunks: &mut Vec<StreamChunk>) {
    if let Some(ref tool_calls) = choice.delta.tool_calls {
        if !text_buf.is_empty() {
            chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
        }
        for tc in tool_calls {
            if let Some(ref func) = tc.function {
                let id = tc.id.clone().unwrap_or_default();
                if let Some(ref name) = func.name {
                    chunks.push(StreamChunk::ToolCallStart {
                        id: id.clone(),
                        name: name.clone(),
                    });
                }
                if let Some(ref args) = func.arguments {
                    chunks.push(StreamChunk::ToolCallDelta {
                        id: id.clone(),
                        arguments_delta: args.clone(),
                    });
                }
            }
        }
    }
}
