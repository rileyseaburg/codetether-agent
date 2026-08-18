//! Quantized model variants supported by the Candle thinker runtime.

use anyhow::{Result, anyhow};
use candle_core::Tensor;
#[cfg(feature = "functiongemma")]
use candle_transformers::models::quantized_gemma3;
use candle_transformers::models::{
    quantized_llama, quantized_qwen2, quantized_qwen3, quantized_qwen3_moe,
};

/// A loaded GGUF model, one variant per supported architecture.
pub(super) enum CandleModel {
    Llama(quantized_llama::ModelWeights),
    Qwen2(quantized_qwen2::ModelWeights),
    Qwen3(quantized_qwen3::ModelWeights),
    Qwen3Moe(quantized_qwen3_moe::GGUFQWenMoE),

    #[cfg(feature = "functiongemma")]
    Gemma3(quantized_gemma3::ModelWeights),
}

impl CandleModel {
    /// Run one forward pass at absolute position `index_pos`.
    pub(super) fn forward(&mut self, x: &Tensor, index_pos: usize) -> Result<Tensor> {
        match self {
            Self::Llama(model) => Ok(model.forward(x, index_pos)?),
            Self::Qwen2(model) => Ok(model.forward(x, index_pos)?),
            Self::Qwen3(model) => Ok(model.forward(x, index_pos)?),
            Self::Qwen3Moe(model) => Ok(model.forward(x, index_pos)?),

            #[cfg(feature = "functiongemma")]
            Self::Gemma3(model) => Ok(model.forward(x, index_pos)?),
        }
    }

    /// Drop cached attention state before an unrelated prompt.
    ///
    /// # Errors
    ///
    /// Returns an error for architectures whose bindings expose no KV reset.
    pub(super) fn reset_kv_cache_for_new_request(&mut self) -> Result<()> {
        match self {
            // quantized_qwen3 uses ConcatKvCache and requires explicit reset.
            Self::Qwen3(model) => {
                model.clear_kv_cache();
                Ok(())
            }
            // quantized_qwen3_moe does not expose KV reset in this build.
            Self::Qwen3Moe(_) => Err(anyhow!(
                "qwen3_moe runtime cannot reset KV cache in this build; restart local runtime or use qwen3"
            )),
            Self::Llama(_) | Self::Qwen2(_) => Ok(()),

            #[cfg(feature = "functiongemma")]
            Self::Gemma3(_) => Ok(()),
        }
    }

    /// Whether a cached prompt prefix can be extended instead of refilled.
    pub(super) fn can_extend_cached_prefix(&self) -> bool {
        true
    }
}
