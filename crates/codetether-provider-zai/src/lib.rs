//! Z.AI completion and SSE transport, independent of the agent runtime.
//!
//! Construct [`ZaiProvider`] with an injected HTTP client and the base URL loaded
//! by your secrets layer. Provider-neutral protocol types live in
//! [`codetether_provider_core`]; the agent adapter retains provider selection.
//!
//! [`application::ZaiProvider`] implements the shared provider trait using the
//! existing application's endpoint, request, and accounting conventions.
//! The root [`ZaiProvider`] retains its injected-client API; both profiles use
//! the protocol services and state machines owned by this crate.
//!
//! ```rust
//! use codetether_provider_zai::ZaiProvider;
//! let provider = ZaiProvider::with_client(reqwest::Client::new(),
//!     String::new(), "http://localhost:8000/api".into());
//! assert_eq!(provider.name(), "zai");
//! ```

pub mod history;
#[path = "catalog/mod.rs"]
pub mod catalog;
mod client;
pub mod completion;
pub mod discovery;
mod error_wire;
mod messages;
mod preview;
pub mod request;
mod response;
mod response_content;
mod response_wire;
#[path = "zai_stream_done.rs"]
mod stream_done;
#[path = "zai_stream_fault.rs"]
mod stream_fault;
#[path = "output/mod.rs"]
pub mod stream_output;
#[path = "stream_request/mod.rs"]
pub mod stream_request;
pub mod thinking;
pub mod tool_arguments;
mod usage;
mod usage_wire;
#[path = "zai_stream_assembly.rs"]
mod zai_stream_assembly;
#[path = "zai_stream_index.rs"]
mod zai_stream_index;
#[path = "zai_stream_state.rs"]
mod zai_stream_state;
#[path = "zai_stream_types.rs"]
mod zai_stream_types;
pub use client::{DEFAULT_BASE_URL, ZaiProvider};
pub use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Role,
    StreamChunk, ToolDefinition, Usage,
};
pub(crate) use usage_wire::ZaiUsage;
pub use zai_stream_index::{capture, delta};

#[cfg(test)]
mod tests;

pub mod streaming;

pub mod application;
