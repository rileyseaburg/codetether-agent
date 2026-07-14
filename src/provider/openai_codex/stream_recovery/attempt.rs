//! Transactional collection of one Codex transport attempt.

use futures::StreamExt;

use super::{Chunks, StreamChunk, classify};

pub(super) enum Outcome {
    Complete(Vec<StreamChunk>),
    Permanent(Vec<StreamChunk>),
    Retry(String),
}

pub(super) async fn collect(mut stream: Chunks) -> Outcome {
    let mut chunks = Vec::new();
    while let Some(chunk) = stream.next().await {
        if let StreamChunk::Error(message) = &chunk {
            if classify::is_retryable(message) {
                return Outcome::Retry(message.clone());
            }
            chunks.push(chunk);
            return Outcome::Permanent(chunks);
        }
        let done = matches!(chunk, StreamChunk::Done { .. });
        chunks.push(chunk);
        if done {
            return Outcome::Complete(chunks);
        }
    }
    Outcome::Retry("transport stream ended before completion".into())
}
