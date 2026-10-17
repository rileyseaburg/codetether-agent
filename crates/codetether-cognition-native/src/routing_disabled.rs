//! No-op tool router for builds without both `functiongemma` and `candle`.
//!
//! Mirrors the real `tool_router` surface so call sites compile identically
//! regardless of which local inference features are enabled.

pub use codetether_cognition_core::tool_routing::disabled_config::ToolRouterConfig;

use anyhow::Result;
use codetether_provider_core::{CompletionResponse, ToolDefinition};

/// Router that never rewrites responses.
///
/// # Examples
/// ```
/// use codetether_cognition_native::disabled_routing::{ToolCallRouter, ToolRouterConfig};
/// assert!(ToolCallRouter::from_config(&ToolRouterConfig::default()).unwrap().is_none());
/// ```
#[derive(Debug, Clone, Default)]
pub struct ToolCallRouter;

impl ToolCallRouter {
    /// Always returns `None`, logging when routing was requested.
    pub fn from_config(config: &ToolRouterConfig) -> Result<Option<Self>> {
        if config.enabled {
            tracing::debug!(target: "codetether_agent::cognition::tool_router",
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
