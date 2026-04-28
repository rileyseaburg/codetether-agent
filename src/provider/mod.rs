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
pub mod local_cuda {
    //! Stub when CUDA is not compiled in.
    use super::*;
    use anyhow::{Result, anyhow};
    use async_trait::async_trait;
    use futures::stream::BoxStream;

    fn feature_error() -> anyhow::Error {
        anyhow!("local_cuda requires --features candle-cuda")
    }

    /// Stub provider type when CUDA support is not compiled in.
    ///
    /// All methods return errors; use `cfg(feature = "candle-cuda")` for the
    /// real implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::local_cuda::LocalCudaProvider;
    /// assert!(!LocalCudaProvider::is_cuda_available());
    /// ```
    pub struct LocalCudaProvider;

    impl LocalCudaProvider {
        pub fn new(_m: String) -> Result<Self> {
            Err(feature_error())
        }
        pub fn with_model(_m: String, _p: String) -> Result<Self> {
            Err(feature_error())
        }
        pub fn with_paths(
            _m: String,
            _p: String,
            _t: Option<String>,
            _a: Option<String>,
        ) -> Result<Self> {
            Err(feature_error())
        }
        pub fn is_cuda_available() -> bool {
            false
        }
        pub fn device_info() -> String {
            "CUDA unavailable".into()
        }
    }

    #[async_trait]
    impl Provider for LocalCudaProvider {
        fn name(&self) -> &str {
            "local_cuda"
        }
        async fn list_models(&self) -> Result<Vec<ModelInfo>> {
            Err(feature_error())
        }
        async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
            Err(feature_error())
        }
        async fn complete_stream(
            &self,
            _: CompletionRequest,
        ) -> Result<BoxStream<'static, StreamChunk>> {
            Err(feature_error())
        }
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct LocalCudaConfig {
        pub model_name: String,
        pub model_path: Option<String>,
        pub context_window: Option<usize>,
        pub max_new_tokens: Option<usize>,
        pub temperature: Option<f32>,
        pub top_p: Option<f32>,
        pub repeat_penalty: Option<f32>,
        pub cuda_device: Option<usize>,
    }

    impl Default for LocalCudaConfig {
        fn default() -> Self {
            Self {
                model_name: "qwen3.5-9b".into(),
                model_path: None,
                context_window: Some(8192),
                max_new_tokens: Some(4096),
                temperature: Some(0.7),
                top_p: Some(0.9),
                repeat_penalty: Some(1.1),
                cuda_device: Some(0),
            }
        }
    }
}

pub mod metrics;
pub mod models;
pub mod moonshot;
pub mod openai;
pub mod openai_codex;
pub mod openrouter;
pub mod retry;
pub mod shared_http;
pub mod stepfun;
pub mod vertex_anthropic;
pub mod vertex_glm;
pub mod zai;

// ── Internal split modules ──────────────────────────────────────────

mod init_dispatch;
mod init_dispatch_impl;
mod init_env;
mod parse;
mod registry;
mod traits;
mod types;

// ── Public re-exports (preserve the original API surface) ───────────

pub use parse::parse_model_string;
pub use registry::ProviderRegistry;
pub use traits::{ModelInfo, Provider};
pub use types::{
    CompletionRequest, CompletionResponse, ContentPart, EmbeddingRequest, EmbeddingResponse,
    FinishReason, Message, Role, StreamChunk, ToolDefinition, Usage,
};

// ── Initialisation modules (impls on ProviderRegistry) ──────────────

mod init_config;
mod init_vault;
