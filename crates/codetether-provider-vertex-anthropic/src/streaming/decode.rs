//! Incremental SSE buffering with unchanged event semantics.
use crate::*;
use futures::StreamExt;
pub(super) fn decode(
    response: reqwest::Response,
) -> futures::stream::BoxStream<'static, StreamChunk> {
    let stream = response.bytes_stream();
    let mut buffer = String::new();

    stream
        .flat_map(move |chunk_result| {
            let mut chunks: Vec<StreamChunk> = Vec::new();
            match chunk_result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    buffer.push_str(&text);

                    let mut text_buf = String::new();

                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].trim().to_string();
                        buffer = buffer[line_end + 1..].to_string();

                        super::line::line(&line, &mut chunks, &mut text_buf);
                    }
                    if !text_buf.is_empty() {
                        chunks.push(StreamChunk::Text(text_buf));
                    }
                }
                Err(e) => chunks.push(StreamChunk::Error(e.to_string())),
            }
            futures::stream::iter(chunks)
        })
        .boxed()
}
