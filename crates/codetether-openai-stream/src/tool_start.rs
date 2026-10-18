//! Encode ToolCallStart provider chunks.
use crate::{Emission, Encoder};
use codetether_openai_wire::openai_stream_chunk;
impl Encoder {
    pub(crate) fn tool_start(&mut self, id: String, name: String) -> Emission {
        self.saw_tool_calls = true;
        let index = self.tool_index(&id);
        let tool_chunk = openai_stream_chunk(
            &self.id,
            self.created,
            &self.model,
            serde_json::json!({
                "tool_calls": [{
                    "index": index,
                    "id": id,
                    "type": "function",
                    "function": {
                        "name": name,
                        "arguments": "",
                    }
                }]
            }),
            None,
        );
        Emission::Event(crate::event::event(&tool_chunk))
    }
}
