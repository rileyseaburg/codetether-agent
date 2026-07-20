//! Server-sent events sourced directly from Rust mux PTY change signals.

use std::{convert::Infallible, time::Duration};

use axum::response::sse::{Event, KeepAlive, Sse};
use futures::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

pub(super) async fn output() -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let receiver = crate::mux::control::subscribe_live_output();
    let events = ReceiverStream::new(receiver).map(|output| {
        let data = serde_json::to_string(&output).unwrap_or_else(|_| "{}".into());
        Ok(Event::default()
            .event("mux")
            .id(format!("{}.{}", output.session, output.offset))
            .data(data))
    });
    Sse::new(events).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}
