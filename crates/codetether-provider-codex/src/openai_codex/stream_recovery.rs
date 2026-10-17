//! Transparent HTTP recovery for an interrupted Codex WebSocket attempt.

use futures::stream::BoxStream;

use super::StreamChunk;

#[path = "stream_recovery/classify.rs"]
mod classify;
#[path = "stream_recovery/factory.rs"]
mod factory;
#[path = "stream_recovery/retry.rs"]
mod retry;

type Chunks = BoxStream<'static, StreamChunk>;

pub(super) use classify::{is_unauthorized, is_upgrade_required, tag_stream_open};
pub(super) use factory::{chatgpt_http, openai_http};

#[cfg(test)]
pub(super) use retry::with_http_retry;
