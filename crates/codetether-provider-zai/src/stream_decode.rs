//! Adapting HTTP byte chunks into provider-neutral streaming events.
use crate::{StreamChunk, sse_state::State};
use futures::{StreamExt, stream::BoxStream};

pub(crate) fn decode(response: reqwest::Response) -> BoxStream<'static, StreamChunk> {
    let mut state = State::default();
    response
        .bytes_stream()
        .flat_map(move |result| {
            let chunks = match result {
                Ok(bytes) => {
                    crate::sse_buffer::append(&mut state, &String::from_utf8_lossy(&bytes))
                }
                Err(error) => crate::sse_transport::error(error),
            };
            futures::stream::iter(chunks)
        })
        .boxed()
}
