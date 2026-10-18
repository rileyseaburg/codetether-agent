//! Provider selection, secret interpretation and constructor dispatch.
//!
//! Implement [Host] for application-owned constructors, then call [dispatch].
//! Provider-specific modules preserve credential and endpoint precedence.
pub mod bonsai;
pub mod cerebras;
pub mod codex;
mod dispatch;
pub mod fallback_policy;
mod host;
mod models;
mod special;
use codetether_provider_anthropic as anthropic;
use codetether_provider_deepseek as deepseek;
use codetether_provider_gemini_web as gemini_web;
use codetether_provider_glm5 as glm5;
use codetether_provider_google as google;
use codetether_provider_moonshot as moonshot;
use codetether_provider_openai as openai;
use codetether_provider_openrouter as openrouter;
use codetether_provider_stepfun as stepfun;
use codetether_provider_vertex_anthropic as vertex_anthropic;
use codetether_provider_vertex_glm as vertex_glm;
pub use dispatch::dispatch;
pub use host::Host;
mod zai {
    pub use codetether_provider_zai::{DEFAULT_BASE_URL, application::ZaiProvider};
}

mod registry;
pub use registry::Registry;
pub mod config;
pub mod environment;
pub mod vault;

/// Shared constructors with caller-provided identity and local runtime hooks.
pub mod application;

#[cfg(test)]
mod chat_image_tests;

#[cfg(test)]
mod routing_identity_tests;
