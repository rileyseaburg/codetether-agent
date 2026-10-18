//! Ordered environment fallback registration without replacing existing keys.
//!
//! The caller applies the fallback security flag before invoking this module.
use crate::Registry;
use anyhow::Result;
use codetether_bedrock_auth::{AwsCredentials, DEFAULT_REGION};
use codetether_provider_core::Provider;
use std::sync::Arc;
mod register_glm5;
mod register_huggingface;
mod register_local_cuda;
mod usai_base_url_from_env;
use register_glm5::register_glm5;
use register_huggingface::register_huggingface;
use register_local_cuda::register_local_cuda;
use usai_base_url_from_env::usai_base_url_from_env;

type Ctor = fn(String) -> Result<Arc<dyn Provider>>;

fn bedrock_bearer<H: crate::Host>(token: String) -> Result<Arc<dyn Provider>> {
    let region = AwsCredentials::detect_region().unwrap_or_else(|| DEFAULT_REGION.into());
    H::bedrock_with_region(token, region)
}
/// Register missing providers from environment configuration in the original order.
///
/// # Arguments
/// * `registry` — Provider collection retaining application instrumentation.
///
/// # Returns
/// Registers valid missing providers and logs individual constructor failures.
/// The caller must enforce its environment-fallback policy before calling.
///
/// # Examples
/// ```no_run
/// # fn load<H: codetether_provider_factory::Host>(registry: &mut impl codetether_provider_factory::Registry) {
/// codetether_provider_factory::environment::register_env_fallbacks::<H>(registry);
/// # }
/// ```
pub fn register_env_fallbacks<H: crate::Host>(registry: &mut impl Registry) {
    let fallbacks: &[(&str, &str, Ctor)] = &[
        ("openai", "OPENAI_API_KEY", |k| {
            Ok(Arc::new(crate::openai::OpenAIProvider::new(k)?))
        }),
        ("anthropic", "ANTHROPIC_API_KEY", |k| {
            Ok(Arc::new(crate::anthropic::AnthropicProvider::new(k)?))
        }),
        ("google", "GOOGLE_API_KEY", |k| {
            Ok(Arc::new(crate::google::GoogleProvider::new(k)?))
        }),
        ("openrouter", "OPENROUTER_API_KEY", |k| {
            Ok(Arc::new(crate::openrouter::OpenRouterProvider::new(k)?))
        }),
        ("bedrock", "AWS_BEARER_TOKEN_BEDROCK", bedrock_bearer::<H>),
        ("zai", "ZAI_API_KEY", |k| {
            Ok(Arc::new(crate::zai::ZaiProvider::with_base_url(
                k,
                crate::zai::DEFAULT_BASE_URL.into(),
            )?))
        }),
        ("github-copilot", "GITHUB_COPILOT_TOKEN", |t| {
            H::copilot_new(t)
        }),
        ("cerebras", "CEREBRAS_API_KEY", |k| {
            crate::cerebras::provider(&k, None)
        }),
        ("usai", "USAI_API_KEY", |k| {
            let base_url = usai_base_url_from_env();
            codetether_provider_tetherscript::usai::provider(&k, base_url.as_deref())
        }),
    ];

    for (pid, env_var, ctor) in fallbacks {
        if !registry.contains(*pid) {
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
    register_local_cuda::<H>(registry);
    crate::bonsai::environment(registry);
}
