//! No-op tool router for builds without both `functiongemma` and `candle`.
//!
//! Mirrors the real `tool_router` surface so call sites compile identically
//! regardless of which local inference features are enabled.

#[path = "tool_router_disabled/config.rs"]
mod config;

pub use config::ToolRouterConfig;

use crate::provider::{CompletionResponse, ToolDefinition};
use anyhow::Result;

/// Router that never rewrites responses.
#[derive(Debug, Clone, Default)]
pub struct ToolCallRouter;

impl ToolCallRouter {
    /// Always returns `None`, logging when routing was requested.
    pub fn from_config(config: &ToolRouterConfig) -> Result<Option<Self>> {
        if config.enabled {
            tracing::debug!(
                "FunctionGemma requested but not compiled in; rebuild with --features functiongemma,candle"
            );
        }
        Ok(None)
    }

    /// Passes `response` through unchanged.
    pub async fn maybe_reformat(
        &self,
        response: CompletionResponse,
        _tools: &[ToolDefinition],
        _model_supports_tools: bool,
    ) -> CompletionResponse {
        response
    }
}
