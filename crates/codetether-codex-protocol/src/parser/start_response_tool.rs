//! start response tool.
use crate::*;
impl CodexProtocol {
    /// Start a tool once its name is known, preserving previously buffered arguments.
    pub fn start_response_tool(
        state: &mut ResponsesToolState,
        name: Option<String>,
        chunks: &mut Vec<StreamChunk>,
    ) {
        if state.name.is_none() {
            state.name = name;
        }
        if state.started {
            return;
        }
        let Some(name) = state.name.clone() else {
            return;
        };
        chunks.push(StreamChunk::ToolCallStart {
            id: state.call_id.clone(),
            name,
        });
        state.started = true;
    }
}
