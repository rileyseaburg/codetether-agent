//! Adapting HTTP byte chunks into provider-neutral streaming events.
use crate::{StreamChunk, streaming::state::State};
use futures::{StreamExt, stream::BoxStream};

pub(crate) fn decode(response: reqwest::Response) -> BoxStream<'static, StreamChunk> {
    decode_with_usage(response, false)
}

pub(super) fn decode_application(response: reqwest::Response) -> BoxStream<'static, StreamChunk> {
    decode_with_usage(response, true)
}

fn decode_with_usage(
    response: reqwest::Response,
    total_prompt_usage: bool,
) -> BoxStream<'static, StreamChunk> {
    let mut state = State {
        total_prompt_usage,
        ..Default::default()
    };
    response
        .bytes_stream()
        .flat_map(move |result| {
            let chunks = match result {
                Ok(bytes) => {
                    crate::streaming::buffer::append(&mut state, &String::from_utf8_lossy(&bytes))
                }
                Err(error) => crate::streaming::transport::error(error),
            };
            futures::stream::iter(chunks)
        })
        .boxed()
}
