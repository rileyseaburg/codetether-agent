//! Responses request payloads built from resolved model choices.
//!
//! The provider resolves runtime configuration before calling CodexProtocol.
//! ResponsesWsBackend selects OpenAI-only controls; instructions and reasoning
//! helpers preserve the same wire shape across HTTP and WebSocket requests.
//!
//! # Examples
//! ```
//! use codetether_codex_protocol::{CodexProtocol, request::ResponsesWsBackend};
//! use codetether_provider_core::CompletionRequest;
//! let request = CompletionRequest {
//!     messages: vec![], tools: vec![], model: "example-model".into(),
//!     temperature: None, top_p: None, max_tokens: Some(128), stop: vec![],
//! };
//! let event = CodexProtocol::build_responses_ws_create_event_for_backend(
//!     &request, &request.model, None, None, ResponsesWsBackend::OpenAi,
//! );
//! assert_eq!(event["max_output_tokens"], 128);
//! let event = CodexProtocol::build_responses_ws_create_event_for_backend(
//!     &request, &request.model, None, None, ResponsesWsBackend::ChatGptCodex,
//! );
//! assert!(event.get("max_output_tokens").is_none());
//! ```
use crate::*;
pub use backend::ResponsesWsBackend;
use codetether_provider_core::{
    CompletionRequest,
    codex::{CodexServiceTier, ThinkingLevel},
};
pub use default_instructions::DEFAULT_RESPONSES_INSTRUCTIONS;
mod apply_service_tier;
mod backend;
mod build_http_responses_body;
mod build_responses_ws_create_event_for_backend;
mod default_instructions;
mod extract_responses_instructions;
mod prompt_cache_key;
pub mod reasoning_request;
