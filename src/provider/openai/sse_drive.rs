//! Byte-stream → [`StreamChunk`] driver for the raw reasoning stream.

use crate::provider::StreamChunk;

use super::sse_lines::drain_lines;

/// Drive a byte stream into ordered [`StreamChunk`]s.
pub(super) fn drive(
    bytes: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>> + Send + 'static,
) -> impl futures::Stream<Item = StreamChunk> + Send + 'static {
    use futures::StreamExt;
    let mut buffer = String::new();
    let mut produced = false;
    let mut usage = None;
    bytes.flat_map(move |result| {
        let mut out = Vec::new();
        match result {
            Ok(b) => {
                buffer.push_str(&String::from_utf8_lossy(&b));
                drain_lines(&mut buffer, &mut produced, &mut usage, &mut out);
            }
            Err(e) => out.push(StreamChunk::Error(e.to_string())),
        }
        futures::stream::iter(out)
    })
}
