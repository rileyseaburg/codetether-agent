//! Execute the tool's request through the shared RLM router.
use super::{ApplicationHost, context};
use codetether_provider_core::Provider;
use codetether_rlm::{RlmConfig, RlmResult};
use codetether_rlm_routing::{Bus, RlmRouter};
use serde_json::Value;
use std::sync::Arc;
#[async_trait::async_trait]
impl<B: Bus> crate::Host for ApplicationHost<B> {
    async fn process(
        content: &str,
        action: &str,
        tool_args: Value,
        provider: Arc<dyn Provider>,
        model: &str,
        config: &RlmConfig,
    ) -> anyhow::Result<RlmResult> {
        RlmRouter::auto_process(
            content,
            context::auto::<B>(provider, model, action, tool_args),
            config,
        )
        .await
    }
}
