//! Thinker backend services with shared inference and transport diagnostics.
//!
//! [Service] implements the cognition client's backend hooks. [Host] supplies
//! credentialed Bedrock construction, signed transport and provider-registry
//! access. HTTP diagnostics and native model dispatch stay in this crate.
use codetether_cognition_core::thinker::{ThinkerConfig, ThinkerOutput};
mod bedrock_backend;
mod build;
mod host;
mod openai_backend;
mod openai_backend_send;
mod openai_backend_status;
mod openai_backend_trace;
mod service;
pub use host::Host;
pub use service::Service;
