//! Z.AI completion and SSE transport, independent of the agent runtime.
//!
//! Construct [`ZaiProvider`] with an injected HTTP client and the base URL loaded
//! by your secrets layer. Provider-neutral protocol types live in
//! [`codetether_provider_core`]; the agent adapter retains provider selection.
//!
//! ```rust
//! use codetether_provider_zai::ZaiProvider;
//! let provider = ZaiProvider::with_client(reqwest::Client::new(),
//!     String::new(), "http://localhost:8000/api".into());
//! assert_eq!(provider.name(), "zai");
//! ```

mod assistant_message;
#[path = "zai_catalog.rs"]
mod catalog;
mod client;
mod complete;
mod discovery;
mod error_wire;
mod messages;
mod model_wire;
mod preview;
mod request;
mod response;
mod response_content;
mod response_wire;
mod sse_buffer;
mod sse_choice;
mod sse_frame;
mod sse_state;
mod sse_transport;
mod stream;
mod stream_decode;
#[path = "zai_stream_done.rs"]
mod stream_done;
#[path = "zai_stream_fault.rs"]
mod stream_fault;
#[path = "zai_stream_output.rs"]
pub mod stream_output;
#[path = "zai_stream_request.rs"]
pub mod stream_request;
mod thinking;
mod tool_arguments;
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
