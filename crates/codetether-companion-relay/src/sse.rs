//! Owner SSE stream: snapshot first, live events, heartbeats, three viewers.
use crate::{ApiError, events::line, runtime::Runtime};
use axum::body::Body;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use futures_util::stream::{self, StreamExt};
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;

/// Register a viewer and return a streaming response.
pub(crate) fn subscribe(rt: &mut Runtime) -> Result<Response, ApiError> {
    rt.viewers.retain(|viewer| !viewer.is_closed());
    if rt.viewers.len() >= 3 {
        return Err(ApiError::new(429, "Too many viewers"));
    }
    let (tx, rx) = mpsc::channel::<String>(256);
    let _ = tx.try_send(line(&rt.snapshot()));
    rt.viewers.push(tx);
    let events = stream::unfold(rx, |mut rx| async move {
        let beat = tokio::time::sleep(Duration::from_secs(10));
        let next = tokio::select! {
            item = rx.recv() => item,
            () = beat => Some(": heartbeat\n\n".to_string()),
        };
        next.map(|text| (Ok::<_, Infallible>(text), rx))
    });
    let headers = [
        (header::CONTENT_TYPE, "text/event-stream"),
        (header::CACHE_CONTROL, "no-store, no-transform"),
        (header::HeaderName::from_static("x-accel-buffering"), "no"),
        (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
    ];
    Ok((StatusCode::OK, headers, Body::from_stream(events.boxed())).into_response())
}
