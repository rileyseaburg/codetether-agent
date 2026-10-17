//! finish responses tool call.
use crate::*;
impl CodexProtocol {
    /// Emit one tool-end chunk for an unfinished known item.
    pub fn finish_responses_tool_call(
        parser: &mut ResponsesSseParser,
        key: &str,
        chunks: &mut Vec<StreamChunk>,
    ) {
        if let Some(state) = parser.tools.get_mut(key)
            && !state.finished
        {
            chunks.push(StreamChunk::ToolCallEnd {
                id: state.call_id.clone(),
            });
            state.finished = true;
        }
    }
}
