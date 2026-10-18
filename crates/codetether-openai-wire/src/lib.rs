//! OpenAI-compatible chat request/response contracts and streamed JSON payloads.
//!
//! Types preserve API serde names and defaults. Stream helpers construct JSON
//! deltas; provider selection, HTTP handling and validation belong to callers.
mod defaults;
mod request;
pub use request::{OpenAiChatCompletionRequest, OpenAiStop};
mod request_message;
pub use request_message::OpenAiRequestMessage;
mod request_call;
pub use request_call::{OpenAiRequestToolCall, OpenAiRequestToolCallFunction};
mod request_tool;
pub use request_tool::{OpenAiRequestTool, OpenAiRequestToolDefinition};
mod response;
pub use response::{OpenAiChatCompletionChoice, OpenAiChatCompletionResponse};
mod response_message;
pub use response_message::OpenAiChatCompletionMessage;
mod response_call;
pub use response_call::{OpenAiResponseToolCall, OpenAiResponseToolCallFunction};
mod usage;
pub use usage::OpenAiUsage;
pub mod stream;
pub use stream::openai_stream_chunk;
