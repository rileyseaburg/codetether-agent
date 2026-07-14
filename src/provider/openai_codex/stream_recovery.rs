//! Transparent HTTP recovery for an interrupted Codex WebSocket attempt.

use futures::stream::BoxStream;

use super::StreamChunk;

#[path = "stream_recovery/attempt.rs"]
mod attempt;
#[path = "stream_recovery/classify.rs"]
mod classify;
#[path = "stream_recovery/factory.rs"]
mod factory;
#[path = "stream_recovery/request_anchor.rs"]
pub(super) mod request_anchor;
#[path = "stream_recovery/retry.rs"]
mod retry;

type Chunks = BoxStream<'static, StreamChunk>;

pub(super) use factory::{chatgpt, chatgpt_http, openai, openai_http};

#[cfg(test)]
pub(super) use retry::with_http_retry;
