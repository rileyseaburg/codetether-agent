//! First-event inspection for one Codex transport attempt.

use anyhow::Result;
use futures::StreamExt;

use super::{Chunks, StreamChunk, classify};

pub(super) enum Outcome {
    Ready(StreamChunk, Chunks),
    Retry(String),
}

pub(super) async fn inspect(mut stream: Chunks) -> Outcome {
    match stream.next().await {
        Some(StreamChunk::Error(message)) if classify::is_retryable(&message) => {
            Outcome::Retry(message)
        }
        Some(first) => Outcome::Ready(first, stream),
        None => Outcome::Retry("transport stream ended before completion".into()),
    }
}

pub(super) async fn open(result: Result<Chunks>) -> Outcome {
    match result {
        Ok(stream) => inspect(stream).await,
        Err(error) if classify::is_retryable(&format!("{error:#}")) => {
            Outcome::Retry(format!("{error:#}"))
        }
        Err(error) => Outcome::Ready(
            StreamChunk::Error(format!("{error:#}")),
            Box::pin(futures::stream::empty()),
        ),
    }
}
