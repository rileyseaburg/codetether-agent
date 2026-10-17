//! OpenAI-compatible chat, streaming, embeddings, and model discovery.

mod imports;
use imports::*;
mod state;
pub use state::OpenAIProvider;
pub mod alias;
mod catalog;
mod chat_messages;
mod completion;
mod constructors;
mod conversions;
mod debug;
mod discovery;
mod embedding;
mod implementation;
mod listing;
mod minimax_error;
mod model_caps;
mod model_entry;
mod openai_models;
mod parse_models;
mod sse_body;
mod sse_drive;
mod sse_frame;
mod sse_lines;
mod sse_msg;
mod sse_msg_assist;
mod sse_repair;
#[cfg(test)]
mod sse_repair_tests;
mod sse_stream;
#[cfg(test)]
mod sse_tests;
mod sse_tool;
#[cfg(test)]
mod sse_tool_tests;
mod sse_types;
mod streaming;
#[cfg(test)]
mod tests;
