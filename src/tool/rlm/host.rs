//! Delegate RLM processing to the application's existing router.
use crate::{
    provider::Provider,
    rlm::{RlmConfig, RlmResult, RlmRouter},
};
use serde_json::Value;
use std::sync::Arc;
/// Existing application router and concrete context adapter.
pub struct ApplicationHost;
#[async_trait::async_trait]
impl codetether_rlm_tool::Host for ApplicationHost {
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
            super::ctx::auto(provider, model, action, tool_args),
            config,
        )
        .await
    }
}
