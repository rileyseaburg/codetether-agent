//! Thinker configuration assembled from environment variables.

use super::defaults::normalize_thinker_endpoint;
use super::env::{env_bool, env_f32, env_u64, env_usize};
use super::thinker_env_candle::{CandleEnv, var};
use super::{ThinkerBackend, ThinkerConfig};

/// Read thinker configuration from `CODETETHER_COGNITION_THINKER_*` variables.
pub(super) fn thinker_config_from_env() -> ThinkerConfig {
    let backend = ThinkerBackend::from_env(
        &var("CODETETHER_COGNITION_THINKER_BACKEND").unwrap_or_else(|| "openai_compat".to_string()),
    );
    let candle = CandleEnv::from_env();
    ThinkerConfig {
        enabled: env_bool("CODETETHER_COGNITION_THINKER_ENABLED", true),
        backend,
        endpoint: normalize_thinker_endpoint(
            &var("CODETETHER_COGNITION_THINKER_BASE_URL")
                .unwrap_or_else(|| "http://127.0.0.1:11434/v1".to_string()),
        ),
        model: var("CODETETHER_COGNITION_THINKER_MODEL")
            .unwrap_or_else(|| "qwen3.5-9b".to_string()),
        api_key: var("CODETETHER_COGNITION_THINKER_API_KEY"),
        temperature: env_f32("CODETETHER_COGNITION_THINKER_TEMPERATURE", 0.2),
        top_p: var("CODETETHER_COGNITION_THINKER_TOP_P").and_then(|v| v.parse().ok()),
        max_tokens: env_usize("CODETETHER_COGNITION_THINKER_MAX_TOKENS", 256),
        timeout_ms: env_u64(
            "CODETETHER_COGNITION_THINKER_TIMEOUT_MS",
            default_timeout_ms(backend),
        ),
        candle_model_path: candle.model_path,
        candle_tokenizer_path: candle.tokenizer_path,
        candle_arch: candle.arch,
        candle_device: candle.device,
        candle_cuda_ordinal: candle.cuda_ordinal,
        candle_repeat_penalty: candle.repeat_penalty,
        candle_repeat_last_n: candle.repeat_last_n,
        candle_seed: candle.seed,
        bedrock_region: var("CODETETHER_COGNITION_THINKER_BEDROCK_REGION")
            .or_else(|| var("AWS_DEFAULT_REGION"))
            .unwrap_or_else(|| "us-east-1".to_string()),
        bedrock_service_tier: var("CODETETHER_COGNITION_THINKER_BEDROCK_SERVICE_TIER")
            .map(|v| v.trim().to_ascii_lowercase())
            .filter(|v| !v.is_empty()),
    }
}

/// Per-backend default request timeout, in milliseconds.
fn default_timeout_ms(backend: ThinkerBackend) -> u64 {
    match backend {
        ThinkerBackend::OpenAICompat => 30_000,
        ThinkerBackend::Candle => 12_000,
        ThinkerBackend::Bedrock | ThinkerBackend::Registry => 60_000,
    }
}
