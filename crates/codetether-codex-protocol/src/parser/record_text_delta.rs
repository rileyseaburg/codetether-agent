//! record text delta.
use crate::*;
impl CodexProtocol {
    /// Append a text chunk when the event contains a string delta.
    pub fn record_text_delta(event: &Value, chunks: &mut Vec<StreamChunk>) {
        if let Some(delta) = event.get("delta").and_then(Value::as_str) {
            chunks.push(StreamChunk::Text(delta.to_string()));
        }
    }
}
