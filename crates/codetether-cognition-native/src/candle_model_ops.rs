//! Forwarding and cache lifecycle for native model variants.
use super::CandleModel;
use anyhow::{Result, anyhow};
use candle_core::Tensor;

impl CandleModel {
    /// Run one forward pass at absolute position `index_pos`.
    pub(in crate::candle) fn forward(&mut self, x: &Tensor, index_pos: usize) -> Result<Tensor> {
        match self {
            Self::Bonsai(model) => model.forward(x, index_pos),
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
    pub(in crate::candle) fn reset_kv_cache_for_new_request(&mut self) -> Result<()> {
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
            Self::Bonsai(model) => {
                model.clear();
                Ok(())
            }
            Self::Llama(_) | Self::Qwen2(_) => Ok(()),

            #[cfg(feature = "functiongemma")]
            Self::Gemma3(_) => Ok(()),
        }
    }

    /// Whether a cached prompt prefix can be extended instead of refilled.
    pub(in crate::candle) fn can_extend_cached_prefix(&self) -> bool {
        !matches!(self, Self::Bonsai(_))
    }
}
