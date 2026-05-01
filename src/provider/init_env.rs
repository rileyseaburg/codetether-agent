//! Environment-variable fallback provider registration.
//!
//! Reads well-known `*_API_KEY` env vars and creates providers that weren't
//! already registered via Vault. Skipped entirely when
//! `CODETETHER_DISABLE_ENV_FALLBACK=1`.

use super::registry::ProviderRegistry;
use crate::provider::traits::Provider;
use anyhow::Result;
use std::sync::Arc;

type Ctor = fn(String) -> Result<Arc<dyn Provider>>;

/// Register providers from environment variables if not already present.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::provider::ProviderRegistry;
/// let mut registry = ProviderRegistry::new();
/// codetether_agent::provider::init_env::register_env_fallbacks(&mut registry);
/// ```
pub fn register_env_fallbacks(registry: &mut ProviderRegistry) {
    let fallbacks: &[(&str, &str, Ctor)] = &[
        ("openai", "OPENAI_API_KEY", |k| {
            Ok(Arc::new(super::openai::OpenAIProvider::new(k)?))
        }),
        ("anthropic", "ANTHROPIC_API_KEY", |k| {
            Ok(Arc::new(super::anthropic::AnthropicProvider::new(k)?))
        }),
        ("google", "GOOGLE_API_KEY", |k| {
            Ok(Arc::new(super::google::GoogleProvider::new(k)?))
        }),
        ("openrouter", "OPENROUTER_API_KEY", |k| {
            Ok(Arc::new(super::openrouter::OpenRouterProvider::new(k)?))
        }),
        ("zai", "ZAI_API_KEY", |k| {
            Ok(Arc::new(super::zai::ZaiProvider::with_base_url(
                k,
                super::zai::DEFAULT_BASE_URL.into(),
            )?))
        }),
        ("github-copilot", "GITHUB_COPILOT_TOKEN", |t| {
            Ok(Arc::new(super::copilot::CopilotProvider::new(t)?))
        }),
        ("cerebras", "CEREBRAS_API_KEY", |k| {
            let src = include_str!("../../examples/tetherscript/cerebras_chat.tether");
            Ok(Arc::new(super::tetherscript_provider::TetherScriptProvider::new(
                src, &k, "https://api.cerebras.ai/v1", "cerebras",
            )?))
        }),
    ];

    for (pid, env_var, ctor) in fallbacks {
        if !registry.providers.contains_key(*pid) {
            if let Ok(key) = std::env::var(env_var) {
                match ctor(key) {
                    Ok(p) => {
                        tracing::info!("Registered {pid} from {env_var} env var");
                        registry.register(p);
                    }
                    Err(e) => tracing::warn!("Failed to init {pid} from env: {e}"),
                }
            }
        }
    }

    register_huggingface(registry);
    register_glm5(registry);
    register_local_cuda(registry);
}

fn register_huggingface(registry: &mut ProviderRegistry) {
    if registry.providers.contains_key("huggingface") {
        return;
    }
    let url = std::env::var("HF_BASE_URL")
        .ok()
        .or_else(|| std::env::var("HUGGINGFACE_BASE_URL").ok())
        .or_else(|| std::env::var("HUGGINGFACE_ENDPOINT").ok());
    let Some(base_url) = url else { return };
    let key = std::env::var("HF_TOKEN")
        .ok()
        .or_else(|| std::env::var("HUGGINGFACE_API_KEY").ok())
        .filter(|v| !v.trim().is_empty());
    match super::openai::OpenAIProvider::with_base_url_optional_key(key, base_url, "huggingface") {
        Ok(p) => {
            tracing::info!("Registered huggingface from HF_BASE_URL env var");
            registry.register(Arc::new(p));
        }
        Err(e) => tracing::warn!("Failed to init huggingface from env: {e}"),
    }
}

fn register_glm5(registry: &mut ProviderRegistry) {
    if registry.providers.contains_key("glm5") {
        return;
    }
    let Ok(base_url) = std::env::var("GLM5_BASE_URL") else {
        return;
    };
    let key = std::env::var("GLM5_API_KEY").unwrap_or_default();
    let model = std::env::var("GLM5_MODEL").unwrap_or_else(|_| super::glm5::DEFAULT_MODEL.into());
    match super::glm5::Glm5Provider::with_model(key, base_url, model) {
        Ok(p) => {
            tracing::info!("Registered glm5 from GLM5_BASE_URL env var");
            registry.register(Arc::new(p));
        }
        Err(e) => tracing::warn!("Failed to init glm5 from env: {e}"),
    }
}

fn register_local_cuda(registry: &mut ProviderRegistry) {
    if registry.providers.contains_key("local_cuda") {
        return;
    }
    let enabled = std::env::var("CODETETHER_LOCAL_CUDA")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let name = std::env::var("LOCAL_CUDA_MODEL")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
        .unwrap_or_else(|_| "qwen3.5-9b".into());
    let path = std::env::var("LOCAL_CUDA_MODEL_PATH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL_PATH"))
        .ok();
    let tok = std::env::var("LOCAL_CUDA_TOKENIZER_PATH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_TOKENIZER_PATH"))
        .ok();
    let arch = std::env::var("LOCAL_CUDA_ARCH")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_ARCH"))
        .ok();
    let has_config = std::env::var("LOCAL_CUDA_MODEL").is_ok()
        || std::env::var("CODETETHER_LOCAL_CUDA_MODEL").is_ok()
        || path.is_some()
        || tok.is_some()
        || arch.is_some();
    if !enabled && !has_config {
        return;
    }
    let result = if let Some(p) = path {
        super::local_cuda::LocalCudaProvider::with_paths(name.clone(), p, tok, arch)
    } else {
        super::local_cuda::LocalCudaProvider::new(name.clone())
    };
    match result {
        Ok(p) => {
            tracing::info!(model = %name, "Registered local_cuda from environment");
            registry.register(Arc::new(p));
        }
        Err(e) => tracing::warn!("Failed to init local_cuda from env: {e}"),
    }
}
