//! HTTP request-opening result classification.

use super::{Chunks, classify};
use anyhow::Result;

pub(super) enum Outcome {
    Ready(Chunks),
    Retry(String),
    Terminal(String, bool),
}

pub(super) async fn open(result: Result<Chunks>) -> Outcome {
    match result {
        Ok(stream) => Outcome::Ready(stream),
        Err(error) if classify::is_retryable_request(&error) => {
            Outcome::Retry(format!("{error:#}"))
        }
        Err(error) => {
            let retryable = classify::terminal_stream_retryable(&error);
            Outcome::Terminal(format!("{error:#}"), retryable)
        }
    }
}
