//! Provider-neutral protocol types and bounded HTTP retry helpers.
//!
//! Build conversation history with [`Message`], submit [`CompletionRequest`]
//! values through a provider, and consume [`CompletionResponse`] or
//! [`StreamChunk`] values. [`retry`] handles transient HTTP failures without
//! depending on a particular provider or the agent runtime.
//!
//! # Examples
//!
//! ```rust
//! use codetether_provider_core::{ContentPart, Message, Role};
//! let message = Message {
//!     role: Role::User,
//!     content: vec![ContentPart::Text { text: "hello".into() }],
//! };
//! assert_eq!(message.role, Role::User);
//! ```

mod completion_request;
mod completion_response;
mod content_part;
mod embedding;
mod finish_reason;
mod message;
mod model_info;
pub mod retry;
mod role;
mod stream_chunk;
mod tool_call;
mod tool_definition;
mod usage;

pub use completion_request::CompletionRequest;
pub use completion_response::CompletionResponse;
pub use content_part::ContentPart;
pub use embedding::{EmbeddingRequest, EmbeddingResponse};
pub use finish_reason::FinishReason;
pub use message::Message;
pub use model_info::ModelInfo;
pub use role::Role;
pub use stream_chunk::StreamChunk;
pub use tool_call::ToolCallRef;
pub use tool_definition::ToolDefinition;
pub use usage::Usage;
