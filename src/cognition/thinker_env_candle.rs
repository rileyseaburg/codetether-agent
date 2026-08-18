//! Candle-specific thinker settings read from the environment.

use super::CandleDevicePreference;
use super::env::{env_f32, env_u64, env_usize};

/// Read an optional environment variable.
pub(super) fn var(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

/// Candle tuning knobs read from `CODETETHER_COGNITION_THINKER_CANDLE_*`.
pub(super) struct CandleEnv {
    pub model_path: Option<String>,
    pub tokenizer_path: Option<String>,
    pub arch: Option<String>,
    pub device: CandleDevicePreference,
    pub cuda_ordinal: usize,
    pub repeat_penalty: f32,
    pub repeat_last_n: usize,
    pub seed: u64,
}

impl CandleEnv {
    /// Read all Candle settings from the environment.
    pub(super) fn from_env() -> Self {
        Self {
            model_path: var("CODETETHER_COGNITION_THINKER_CANDLE_MODEL_PATH"),
            tokenizer_path: var("CODETETHER_COGNITION_THINKER_CANDLE_TOKENIZER_PATH"),
            arch: var("CODETETHER_COGNITION_THINKER_CANDLE_ARCH"),
            device: CandleDevicePreference::from_env(
                &var("CODETETHER_COGNITION_THINKER_CANDLE_DEVICE")
                    .unwrap_or_else(|| "auto".to_string()),
            ),
            cuda_ordinal: env_usize("CODETETHER_COGNITION_THINKER_CANDLE_CUDA_ORDINAL", 0),
            repeat_penalty: env_f32("CODETETHER_COGNITION_THINKER_CANDLE_REPEAT_PENALTY", 1.1),
            repeat_last_n: env_usize("CODETETHER_COGNITION_THINKER_CANDLE_REPEAT_LAST_N", 64),
            seed: env_u64("CODETETHER_COGNITION_THINKER_CANDLE_SEED", 42),
        }
    }
}
