//! Application RLM-router boundary.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Provider;
use codetether_rlm::{RlmConfig, RlmResult};
use serde_json::Value;
use std::sync::Arc;
/// Existing application router used to process the collected content.
///
/// # Implementors
/// Preserve the application session identifier, trace identity and routing
/// context when forwarding the tool's provider, model and generated arguments.
#[async_trait]
pub trait Host: Send + Sync {
    /// Process content with the application's existing RLM context.
    ///
    /// # Errors
    /// Returns router errors; the tool renders its established fallback response.
    async fn process(
        content: &str,
        action: &str,
        tool_args: Value,
        provider: Arc<dyn Provider>,
        model: &str,
        config: &RlmConfig,
    ) -> Result<RlmResult>;
}
