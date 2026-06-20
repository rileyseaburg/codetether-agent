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
//! - [`init_vault`] — Vault-based provider initialization
//! - [`init_config`] — TOML-config-based initialization
//! - [`init_env`] — environment-variable fallback
//! - [`init_dispatch`] / [`init_dispatch_impl`] — per-provider constructors
//!
//! Provider implementations live in their own modules (`openai`, `anthropic`,
//! `bedrock/`, etc.).

// ── Sub-module declarations ─────────────────────────────────────────

pub mod anthropic;
pub mod bedrock;
pub mod body_cap;
pub mod codex_reasoning;
pub mod copilot;
pub mod deepseek;
mod fallback_policy;
pub mod gemini_web;
pub mod glm5;
pub mod google;
pub mod limits;
#[cfg(feature = "candle-cuda")]
pub mod local_cuda;
pub mod pricing;
pub mod util;
#[cfg(not(feature = "candle-cuda"))]
#[allow(dead_code)]
#[path = "local_cuda_nocuda.rs"]
pub mod local_cuda;

pub mod metrics;
pub mod models;
pub mod moonshot;
pub mod openai;
pub mod openai_codex;
pub mod openrouter;
pub mod retry;
pub mod shared_http;
pub mod stepfun;
pub mod tetherscript_provider;
pub mod vertex_anthropic;
pub mod vertex_glm;
pub mod zai;
mod zai_merge;
// ── Internal split modules ──────────────────────────────────────────

mod init_dispatch;
mod init_dispatch_impl;
mod init_env;
mod parse;
mod registry;
mod tenant_keys;
mod tool_call;
mod traits;
mod types;

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

mod init_config;
mod init_vault;