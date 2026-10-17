//! Loopback-only regression tests of the public provider interface.

use super::OpenAIProvider;
use codetether_provider_core::{
    CompletionRequest, ContentPart, EmbeddingRequest, Message, Provider, Role, StreamChunk,
    ToolDefinition,
};
use futures::StreamExt;
use serde_json::{Value, json};

struct Recorded {
    path: String,
    headers: String,
    body: Value,
}
struct Reply {
    status: u16,
    content_type: &'static str,
    body: String,
    declared_len: Option<usize>,
}
fn reply(status: u16, content_type: &'static str, body: String) -> Reply {
    Reply {
        status,
        content_type,
        body,
        declared_len: None,
    }
}
mod server;
use server::serve;
mod receive;
mod responses;
use responses::{chat, frame};
mod completion;
mod discovery;
mod embedding;
mod reasoning_stream;
mod typed_stream;

mod request;
use request::request;
