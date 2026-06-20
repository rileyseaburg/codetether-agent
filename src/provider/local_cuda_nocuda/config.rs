//! `LocalCudaConfig` for builds without CUDA support compiled in.
//!
//! Mirrors the CUDA-backed configuration so config loading compiles
//! identically regardless of the `candle-cuda` feature.

/// Configuration for the CUDA-backed local provider.
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
