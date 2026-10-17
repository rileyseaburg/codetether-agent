//! Shared local-provider configuration for CUDA and non-CUDA builds.

/// Configuration shared by local CUDA and non-CUDA provider builds.
///
/// # Examples
/// ```
/// let config = codetether_provider_core::local::LocalCudaConfig::default();
/// assert_eq!(config.model_name, "qwen3.5-9b");
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalCudaConfig {
    /// Model name (e.g., "qwen3.5-9b", "deepseek-coder-6.7b")
    pub model_name: String,
    /// Path to the model weights (GGUF or safetensors format)
    pub model_path: Option<String>,
    /// Context window size
    pub context_window: Option<usize>,
    /// Maximum tokens to generate
    pub max_new_tokens: Option<usize>,
    /// Temperature for sampling
    pub temperature: Option<f32>,
    /// Top-p for nucleus sampling
    pub top_p: Option<f32>,
    /// Repetition penalty
    pub repeat_penalty: Option<f32>,
    /// CUDA device ordinal (0 for first GPU)
    pub cuda_device: Option<usize>,
}

impl Default for LocalCudaConfig {
    fn default() -> Self {
        Self {
            model_name: "qwen3.5-9b".to_string(),
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
