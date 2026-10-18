//! Render real SSE framing for contract assertions.

use axum::response::{IntoResponse, Sse, sse::Event};
use futures::{Stream, StreamExt};
use std::{convert::Infallible, time::Duration};

pub(super) async fn first<S>(stream: S) -> String
where
    S: Stream<Item = Result<Event, Infallible>> + Send + 'static,
{
    events(stream, 1).await
}

pub(super) async fn events<S>(stream: S, count: usize) -> String
where
    S: Stream<Item = Result<Event, Infallible>> + Send + 'static,
{
    let body = Sse::new(stream.take(count)).into_response().into_body();
    let bytes = tokio::time::timeout(Duration::from_secs(1), axum::body::to_bytes(body, 8192))
        .await
        .expect("worker stream must not stall")
        .expect("SSE body must be readable");
    String::from_utf8(bytes.to_vec()).expect("SSE must be UTF-8")
}
