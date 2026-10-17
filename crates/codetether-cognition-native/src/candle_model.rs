//! Quantized model variants supported by the Candle thinker runtime.

#[cfg(feature = "functiongemma")]
use candle_transformers::models::quantized_gemma3;
use candle_transformers::models::{
    quantized_llama, quantized_qwen2, quantized_qwen3, quantized_qwen3_moe,
};

/// A loaded GGUF model, one variant per supported architecture.
pub(super) enum CandleModel {
    Bonsai(super::bonsai::Model),
    Llama(quantized_llama::ModelWeights),
    Qwen2(quantized_qwen2::ModelWeights),
    Qwen3(quantized_qwen3::ModelWeights),
    Qwen3Moe(quantized_qwen3_moe::GGUFQWenMoE),

    #[cfg(feature = "functiongemma")]
    Gemma3(quantized_gemma3::ModelWeights),
}
#[path = "candle_model_ops.rs"]
mod ops;
