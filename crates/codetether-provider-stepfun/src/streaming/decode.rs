//! HTTP byte chunks to the existing single-event stream.
use crate::*;
pub(crate) fn decode(
    response: reqwest::Response,
) -> futures::stream::BoxStream<'static, StreamChunk> {
    response
        .bytes_stream()
        .map(|result| match result {
            Ok(bytes) => {
                let text = String::from_utf8_lossy(&bytes);
                let mut chunks = Vec::new();
                for line in text.lines() {
                    super::line::push(line, &mut chunks);
                }
                super::select::first(chunks)
            }
            Err(e) => StreamChunk::Error(e.to_string()),
        })
        .boxed()
}
