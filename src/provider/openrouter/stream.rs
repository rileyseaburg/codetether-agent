//! OpenRouter streaming parser and byte-stream adapter.
//!
//! OpenAI-compatible tool deltas identify a call by `index`. Only the first
//! delta normally carries `id` and `name`; later deltas carry argument fragments
//! with `id: null`. The old stateless parser replaced missing IDs with `""`,
//! creating a second placeholder tool named `"tool"` while leaving the real
//! call with empty arguments. [`Parser`] persists identity by index instead.

#[path = "stream/event.rs"]
mod event;
#[path = "stream/finish.rs"]
mod finish;
#[path = "stream/parser.rs"]
mod parser;
#[path = "stream/tool_state.rs"]
mod tool_state;
#[path = "stream/types.rs"]
mod types;

use crate::provider::StreamChunk;
use futures::{Stream, StreamExt};

use parser::Parser;

pub(super) fn adapt<S, E>(stream: S) -> futures::stream::BoxStream<'static, StreamChunk>
where
    S: Stream<Item = Result<bytes::Bytes, E>> + Send + 'static,
    E: std::fmt::Display + Send + 'static,
{
    let mut parser = Parser::default();
    stream
        .flat_map(move |result| {
            let chunks = match result {
                Ok(bytes) => parser.push(&bytes),
                Err(error) => vec![StreamChunk::Error(error.to_string())],
            };
            futures::stream::iter(chunks)
        })
        .boxed()
}

#[cfg(test)]
#[path = "stream_tests.rs"]
mod tests;
