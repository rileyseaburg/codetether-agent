//! Stream encoded provider events through Axum SSE.
use axum::response::{
    IntoResponse, Response,
    sse::{Event, KeepAlive, Sse},
};
use codetether_provider_core::StreamChunk;
use futures::{StreamExt, stream::BoxStream};
use std::{convert::Infallible, time::Duration};
pub(crate) fn response(
    mut provider_stream: BoxStream<'static, StreamChunk>,
    chat_id: String,
    now: i64,
    openai_model: String,
) -> Response {
    let (stream_id, stream_model) = (chat_id.clone(), openai_model.clone());
    let event_stream = async_stream::stream! {
        let mut encoder = codetether_openai_stream::Encoder::new(stream_id, now, stream_model);
        yield Ok::<Event, Infallible>(encoder.start());
        while let Some(chunk) = provider_stream.next().await {
            match encoder.push(chunk) {
                codetether_openai_stream::Emission::Skip => {},
                codetether_openai_stream::Emission::Event(event) => yield Ok(event),
                codetether_openai_stream::Emission::Error(event) => {
                    yield Ok(event);
                    yield Ok(Event::default().data("[DONE]"));
                    return;
                }
            }
        }
        yield Ok(encoder.finish());
        yield Ok(Event::default().data("[DONE]"));
    };

    Sse::new(event_stream)
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
        .into_response()
}
