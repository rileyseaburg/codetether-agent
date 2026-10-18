//! Encode ToolCallDelta provider chunks.
use crate::{Emission, Encoder};
use codetether_openai_wire::openai_stream_chunk;
impl Encoder {
    pub(crate) fn tool_delta(&mut self, id: String, arguments_delta: String) -> Emission {
        if arguments_delta.is_empty() {
            return Emission::Skip;
        }
        self.saw_tool_calls = true;
        let index = self.tool_index(&id);
        let tool_delta_chunk = openai_stream_chunk(
            &self.id,
            self.created,
            &self.model,
            serde_json::json!({
                "tool_calls": [{
                    "index": index,
                    "id": id,
                    "type": "function",
                    "function": {
                        "arguments": arguments_delta,
                    }
                }]
            }),
            None,
        );
        Emission::Event(crate::event::event(&tool_delta_chunk))
    }
}
