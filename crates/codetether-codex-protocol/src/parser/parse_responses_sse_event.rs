//! parse responses sse event.
use crate::*;
impl CodexProtocol {
    /// Decode one SSE data payload; ignore malformed JSON and the DONE marker.
    pub fn parse_responses_sse_event(
        parser: &mut ResponsesSseParser,
        data: &str,
        chunks: &mut Vec<StreamChunk>,
    ) {
        if data == "[DONE]" {
            return;
        }

        let Ok(event): Result<Value, _> = serde_json::from_str(data) else {
            return;
        };

        Self::parse_responses_event(parser, &event, chunks);
    }
}
