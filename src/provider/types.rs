//! Compatibility exports for provider-neutral conversation and completion types.
//!
//! See [`codetether_provider_core`] for construction examples and wire formats.

pub use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, EmbeddingRequest, EmbeddingResponse,
    FinishReason, Message, Role, StreamChunk, ToolDefinition, Usage,
};
