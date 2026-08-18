//! Configuration contract for [`ThinkerClient`](super::ThinkerClient).

use super::{CandleDevicePreference, ThinkerBackend};

/// Configuration for [`ThinkerClient`](super::ThinkerClient).
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::ThinkerConfig;
/// let cfg = ThinkerConfig::default();
/// assert!(!cfg.enabled);
/// ```
#[derive(Debug, Clone)]
pub struct ThinkerConfig {
    pub enabled: bool,
    pub backend: ThinkerBackend,
    pub endpoint: String,
    pub model: String,
    pub api_key: Option<String>,
    pub temperature: f32,
    pub top_p: Option<f32>,
    pub max_tokens: usize,
    pub timeout_ms: u64,
    pub candle_model_path: Option<String>,
    pub candle_tokenizer_path: Option<String>,
    pub candle_arch: Option<String>,
    pub candle_device: CandleDevicePreference,
    pub candle_cuda_ordinal: usize,
    pub candle_repeat_penalty: f32,
    pub candle_repeat_last_n: usize,
    pub candle_seed: u64,
    pub bedrock_region: String,
    pub bedrock_service_tier: Option<String>,
}

impl Default for ThinkerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            backend: ThinkerBackend::OpenAICompat,
            endpoint: "http://127.0.0.1:11434/v1/chat/completions".to_string(),
            model: "qwen3.5-9b".to_string(),
            api_key: None,
            temperature: 0.2,
            top_p: None,
            max_tokens: 256,
            timeout_ms: 30_000,
            candle_model_path: None,
            candle_tokenizer_path: None,
            candle_arch: None,
            candle_device: CandleDevicePreference::Auto,
            candle_cuda_ordinal: 0,
            candle_repeat_penalty: 1.1,
            candle_repeat_last_n: 64,
            candle_seed: 42,
            bedrock_region: "us-east-1".to_string(),
            bedrock_service_tier: None,
        }
    }
}
