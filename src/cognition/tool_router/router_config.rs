//! Bind the router's thinker configuration to its selected checkpoint.

use super::ToolRouterConfig;
use crate::cognition::{ThinkerBackend, ThinkerConfig};

/// Use the configured checkpoint as identity, never the unrelated thinker default.
pub(super) fn thinker_config(
    config: &ToolRouterConfig,
    model_path: &str,
    tokenizer_path: &str,
) -> ThinkerConfig {
    ThinkerConfig {
        enabled: true,
        backend: ThinkerBackend::Candle,
        model: model_path.to_owned(),
        candle_model_path: Some(model_path.to_owned()),
        candle_tokenizer_path: Some(tokenizer_path.to_owned()),
        candle_arch: Some(config.arch.clone()),
        candle_device: config.device,
        max_tokens: config.max_tokens,
        temperature: config.temperature,
        ..ThinkerConfig::default()
    }
}
