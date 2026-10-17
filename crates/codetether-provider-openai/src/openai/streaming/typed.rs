//! typed for the OpenAI-compatible transport.

use super::*;

pub(super) fn typed_response_chunks(
    response: async_openai::types::chat::CreateChatCompletionStreamResponse,
) -> Vec<StreamChunk> {
    let mut out = Vec::new();
    if let Some(choice) = response.choices.first() {
        // Text content delta
        if let Some(content) = &choice.delta.content {
            if !content.is_empty() {
                out.push(StreamChunk::Text(content.clone()));
            }
        }
        // Tool call deltas
        if let Some(tool_calls) = &choice.delta.tool_calls {
            for tc in tool_calls {
                if let Some(func) = &tc.function {
                    // First chunk for a tool call has id and name
                    if let Some(id) = &tc.id {
                        out.push(StreamChunk::ToolCallStart {
                            id: id.clone(),
                            name: func.name.clone().unwrap_or_default(),
                        });
                    }
                    // Argument deltas
                    if let Some(args) = &func.arguments {
                        if !args.is_empty() {
                            // Derive the id from tc.id or use index as fallback
                            let id = tc
                                .id
                                .clone()
                                .unwrap_or_else(|| format!("tool_{}", tc.index));
                            out.push(StreamChunk::ToolCallDelta {
                                id,
                                arguments_delta: args.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    out
}
