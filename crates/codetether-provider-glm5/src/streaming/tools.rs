//! Streaming function starts and argument fragments.
use crate::*;
pub(super) fn push(
    choice: &Glm5StreamChoice,
    text_buf: &mut String,
    chunks: &mut Vec<StreamChunk>,
) {
    if let Some(ref tool_calls) = choice.delta.tool_calls {
        if !text_buf.is_empty() {
            chunks.push(StreamChunk::Text(std::mem::take(text_buf)));
        }
        for tc in tool_calls {
            if let Some(ref func) = tc.function {
                if let Some(ref name) = func.name {
                    chunks.push(StreamChunk::ToolCallStart {
                        id: tc.id.clone().unwrap_or_default(),
                        name: name.clone(),
                    });
                }
                if let Some(ref args) = func.arguments {
                    let delta = match args {
                        Value::String(s) => s.clone(),
                        other => serde_json::to_string(other).unwrap_or_default(),
                    };
                    if !delta.is_empty() {
                        chunks.push(StreamChunk::ToolCallDelta {
                            id: tc.id.clone().unwrap_or_default(),
                            arguments_delta: delta,
                        });
                    }
                }
            }
        }
    }
}
