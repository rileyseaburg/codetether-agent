//! Encode Text provider chunks.
use crate::{Emission, Encoder};
use codetether_openai_wire::openai_stream_chunk;
impl Encoder {
    pub(crate) fn text(&mut self, text: String) -> Emission {
        if text.is_empty() {
            return Emission::Skip;
        }
        self.saw_text = true;
        let content_chunk = openai_stream_chunk(
            &self.id,
            self.created,
            &self.model,
            serde_json::json!({ "content": text }),
            None,
        );
        Emission::Event(crate::event::event(&content_chunk))
    }
}
