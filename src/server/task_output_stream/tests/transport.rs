//! Render finite broadcast streams through the actual Axum SSE response body.

use crate::bus::BusEnvelope;
use axum::response::{IntoResponse, Sse};
use std::time::Duration;

pub(super) async fn render(messages: Vec<BusEnvelope>, capacity: usize) -> String {
    let (tx, rx) = tokio::sync::broadcast::channel(capacity);
    for message in messages {
        tx.send(message).unwrap();
    }
    drop(tx);
    let body = Sse::new(super::super::service::events(rx, "abc".into()))
        .into_response()
        .into_body();
    let bytes = tokio::time::timeout(Duration::from_secs(2), axum::body::to_bytes(body, 8192))
        .await
        .expect("closed bus must finish SSE")
        .unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}
