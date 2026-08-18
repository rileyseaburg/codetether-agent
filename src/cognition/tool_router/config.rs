//! Configuration contract for the FunctionGemma tool-call router.

use crate::cognition::CandleDevicePreference;

/// Environment-variable driven configuration for the tool-call router.
#[derive(Debug, Clone)]
pub struct ToolRouterConfig {
    /// Whether the router is active. Default: `false`.
    pub enabled: bool,
    /// Filesystem path to the FunctionGemma GGUF model.
    pub model_path: Option<String>,
    /// Filesystem path to the matching `tokenizer.json`.
    pub tokenizer_path: Option<String>,
    /// Architecture hint (default: `"gemma3"`).
    pub arch: String,
    /// Device preference (auto / cpu / cuda).
    pub device: CandleDevicePreference,
    /// Max tokens for the FunctionGemma response.
    ///
    /// FunctionGemma only outputs `<tool_call>` JSON blocks, so 128 is generous.
    pub max_tokens: usize,
    /// Temperature for FunctionGemma sampling.
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
