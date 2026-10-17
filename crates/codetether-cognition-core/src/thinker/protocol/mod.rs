//! Thinker wire contracts and backend request/response conversion.
//!
//! These modules format OpenAI-compatible, Bedrock and registry-provider
//! messages. Transport, authentication and backend execution remain in the host.
use super::{ThinkerConfig, ThinkerOutput};
pub mod bedrock_request;
pub mod bedrock_response;
pub mod openai_request;
pub mod openai_response;
pub mod openai_wire;
pub mod provider_output;
pub mod provider_request;
