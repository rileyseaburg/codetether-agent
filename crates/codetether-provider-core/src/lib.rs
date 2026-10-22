//! Provider contracts, protocol types, model identities, and bounded HTTP retries.
//!
//! Build conversation history with [`Message`], submit [`CompletionRequest`]
//! values through a provider, and consume [`CompletionResponse`] or
//! [`StreamChunk`] values. [`retry`] handles transient HTTP failures without
//! depending on transport adapters or the agent runtime. Shared model rules and
//! catalog caches keep request selection and reported model identities consistent.
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
pub mod runtime_context;
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

mod provider;
pub use provider::Provider;
pub mod codex;
pub mod model_identity;

#[cfg(test)]
mod provider_tests;

pub mod body_cap;
pub mod chat_images;
pub mod shared_http;
pub mod socket_opts;

#[cfg(test)]
mod body_cap_tests;

/// Shared context-window limits with model-specific precedence.
pub mod limits;

pub mod request_identity;

pub mod local;

pub mod pricing;

pub mod routing;

/// Compact tool schemas and measure serialized schema bytes.
pub mod tool_schema;
