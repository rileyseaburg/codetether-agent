//! Path and architecture resolution for Candle model loading.

use anyhow::{Result, anyhow};
use candle_core::quantized::gguf_file;

use super::ThinkerConfig;

/// Context window assumed when GGUF metadata omits it.
pub(super) const DEFAULT_CONTEXT_WINDOW: usize = 4096;

/// Resolve the required model and tokenizer paths.
///
/// # Errors
///
/// Returns an error when either path is unset.
pub(super) fn paths(config: &ThinkerConfig) -> Result<(&str, &str)> {
    let model = config.candle_model_path.as_deref().ok_or_else(|| {
        anyhow!("candle backend requires CODETETHER_COGNITION_THINKER_CANDLE_MODEL_PATH")
    })?;
    let tokenizer = config.candle_tokenizer_path.as_deref().ok_or_else(|| {
        anyhow!("candle backend requires CODETETHER_COGNITION_THINKER_CANDLE_TOKENIZER_PATH")
    })?;
    Ok((model, tokenizer))
}

/// Prefer the configured architecture, else GGUF metadata, else llama.
pub(super) fn architecture(config: &ThinkerConfig, content: &gguf_file::Content) -> String {
    config
        .candle_arch
        .clone()
        .or_else(|| {
            content
                .metadata
                .get("general.architecture")
                .and_then(|v| v.to_string().ok())
                .cloned()
        })
        .unwrap_or_else(|| "llama".to_string())
        .to_ascii_lowercase()
}
