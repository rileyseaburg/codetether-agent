//! Router configuration for builds without both `functiongemma` and `candle`.

use crate::cognition::CandleDevicePreference;

/// Configuration accepted, and ignored, when the real router is not compiled in.
#[derive(Debug, Clone)]
pub struct ToolRouterConfig {
    pub enabled: bool,
    pub model_path: Option<String>,
    pub tokenizer_path: Option<String>,
    pub arch: String,
    pub device: CandleDevicePreference,
    pub max_tokens: usize,
    pub temperature: f32,
}

impl Default for ToolRouterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model_path: None,
            tokenizer_path: None,
            arch: "gemma3".to_string(),
            device: CandleDevicePreference::Auto,
            max_tokens: 128,
            temperature: 0.1,
        }
    }
}

impl ToolRouterConfig {
    /// Read configuration from the environment; always disabled here.
    pub fn from_env() -> Self {
        Self::default()
    }
}
