//! Router construction from a [`ToolRouterConfig`](super::ToolRouterConfig).

use anyhow::{Result, anyhow};
use std::sync::{Arc, Mutex};

use super::{ToolCallRouter, ToolRouterConfig};
use crate::cognition::thinker::CandleRuntime;
use crate::cognition::{ThinkerBackend, ThinkerConfig};

impl ToolCallRouter {
    /// Construct from a [`ToolRouterConfig`].
    ///
    /// # Errors
    ///
    /// Returns an error when the router is enabled but the model or tokenizer
    /// path is missing, or when the Candle runtime fails to load.
    ///
    /// Returns `Ok(None)` when the router is disabled.
    pub fn from_config(config: &ToolRouterConfig) -> Result<Option<Self>> {
        if !config.enabled {
            tracing::debug!("FunctionGemma tool router is disabled");
            return Ok(None);
        }
        let model_path = config.model_path.as_ref().ok_or_else(|| {
            anyhow!("CODETETHER_TOOL_ROUTER_MODEL_PATH is required when the tool router is enabled")
        })?;
        let tokenizer_path = config.tokenizer_path.as_ref().ok_or_else(|| {
            anyhow!(
                "CODETETHER_TOOL_ROUTER_TOKENIZER_PATH is required when the tool router is enabled"
            )
        })?;

        let runtime = CandleRuntime::new(&thinker_config(config, model_path, tokenizer_path))?;
        tracing::info!(
            model_path = %model_path,
            arch = %config.arch,
            "FunctionGemma tool-call router initialised"
        );
        Ok(Some(Self {
            runtime: Arc::new(Mutex::new(runtime)),
        }))
    }
}

/// Build a Candle-backed thinker config for the FunctionGemma model.
fn thinker_config(
    config: &ToolRouterConfig,
    model_path: &str,
    tokenizer_path: &str,
) -> ThinkerConfig {
    ThinkerConfig {
        enabled: true,
        backend: ThinkerBackend::Candle,
        candle_model_path: Some(model_path.to_string()),
        candle_tokenizer_path: Some(tokenizer_path.to_string()),
        candle_arch: Some(config.arch.clone()),
        candle_device: config.device,
        max_tokens: config.max_tokens,
        temperature: config.temperature,
        ..ThinkerConfig::default()
    }
}
