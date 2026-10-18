//! AI Provider abstraction layer.
//!
//! Unified interface for multiple AI providers (OpenAI, Anthropic, Google,
//! StepFun, Bedrock, etc.).
//!
//! # Architecture
//!
//! - [`types`] — shared data types (`Message`, `StreamChunk`, etc.)
//! - [`traits`] — the `Provider` trait and `ModelInfo`
//! - [`registry`] — `ProviderRegistry` (name → provider map)
//! - [`parse`] — model-string parser (`"openai/gpt-4o"` → `(provider, model)`)
//! - [`codetether_application_providers::ProviderRegistry`] — shared Vault,
//!   configuration and task-key initialization using the native factory
//! - [`init_env`] — environment-variable fallback
//! - [`init_dispatch`] / [`init_dispatch_impl`] — per-provider constructors
//!
//! Provider implementations live in their own modules (`openai`, `anthropic`,
//! `bedrock/`, etc.).

// ── Sub-module declarations ─────────────────────────────────────────

pub use codetether_provider_anthropic as anthropic;
pub use codetether_provider_bedrock::provider as bedrock;
pub use codetether_provider_core::body_cap;
pub mod codex_reasoning;
pub mod copilot;
pub use codetether_provider_deepseek as deepseek;
mod fallback_policy;
pub use codetether_provider_core::limits;
pub use codetether_provider_gemini_web as gemini_web;
pub use codetether_provider_glm5 as glm5;
pub use codetether_provider_google as google;
include!("native_modules.rs");
pub mod pricing;
include!("image_modules.rs");

pub mod metrics;
pub use codetether_model_catalog as models;
pub use codetether_provider_moonshot as moonshot;
pub use codetether_provider_openai as openai;
pub mod openai_codex;
pub use codetether_provider_openrouter as openrouter;
pub mod retry;
pub use codetether_provider_stepfun as stepfun;
pub mod tetherscript_provider;
pub use codetether_provider_vertex_anthropic as vertex_anthropic;
pub use codetether_provider_vertex_glm as vertex_glm;
pub mod zai;
// ── Internal split modules ──────────────────────────────────────────

mod init_dispatch;
use codetether_provider_factory as init_dispatch_impl;
mod init_env;
mod parse;
#[doc(hidden)]
pub mod registry;
mod tenant_keys;
use codetether_provider_core as tool_call;
use codetether_provider_core as traits;
use codetether_provider_core as types;

// ── Public re-exports (preserve the original API surface) ───────────

pub use parse::parse_model_string;
pub use registry::ProviderRegistry;
pub use tenant_keys::{PerTaskProviderKeys, TenantProviderKeyPayload};
pub use tool_call::ToolCallRef;
pub use traits::{ModelInfo, Provider};
pub use types::{
    CompletionRequest, CompletionResponse, ContentPart, EmbeddingRequest, EmbeddingResponse,
    FinishReason, Message, Role, StreamChunk, ToolDefinition, Usage,
};

// ── Initialisation modules (impls on ProviderRegistry) ──────────────
