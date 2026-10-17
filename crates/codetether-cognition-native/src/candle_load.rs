//! GGUF weight loading and architecture dispatch for the Candle runtime.

use anyhow::{Context, Result};
use candle_core::quantized::gguf_file;
use candle_core::{DType, Device};
use std::io::BufReader;

#[cfg(feature = "functiongemma")]
use candle_transformers::models::quantized_gemma3;
use candle_transformers::models::{
    quantized_llama, quantized_qwen2, quantized_qwen3, quantized_qwen3_moe,
};

use super::candle_arch::unsupported;
use super::candle_model::CandleModel;

/// Open GGUF reader positioned after the metadata header.
type Reader<'a> = &'a mut BufReader<std::fs::File>;

/// Load GGUF weights for `architecture` from an open reader.
///
/// # Errors
///
/// Returns an error for unsupported architectures or malformed weights.
pub(super) fn load(
    architecture: &str,
    content: gguf_file::Content,
    reader: Reader<'_>,
    device: &Device,
    path: &str,
) -> Result<CandleModel> {
    let ctx = || format!("failed to load {architecture} gguf from {path}");
    match architecture {
        "llama" => Ok(CandleModel::Llama(
            quantized_llama::ModelWeights::from_gguf(content, reader, device).with_context(ctx)?,
        )),
        "qwen2" => Ok(CandleModel::Qwen2(
            quantized_qwen2::ModelWeights::from_gguf(content, reader, device).with_context(ctx)?,
        )),
        "qwen3" => Ok(CandleModel::Qwen3(
            quantized_qwen3::ModelWeights::from_gguf(content, reader, device).with_context(ctx)?,
        )),
        "qwen3moe" | "qwen3_moe" => Ok(CandleModel::Qwen3Moe(
            quantized_qwen3_moe::GGUFQWenMoE::from_gguf(content, reader, device, DType::F16)
                .with_context(ctx)?,
        )),
        #[cfg(feature = "functiongemma")]
        arch if super::candle_arch::GEMMA.contains(&arch) => Ok(CandleModel::Gemma3(
            quantized_gemma3::ModelWeights::from_gguf(content, reader, device).with_context(ctx)?,
        )),
        other => Err(unsupported(other)),
    }
}
