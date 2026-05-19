use super::tool_impl::AgentTool;
use crate::provider::{CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk};
use crate::tool::Tool;
use anyhow::Result;
use async_trait::async_trait;
use futures::stream::BoxStream;
use serde_json::json;
use std::sync::Arc;

struct MockProvider;

#[async_trait]
impl Provider for MockProvider {
    fn name(&self) -> &str {
        "mock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![ModelInfo {
            id: "paid".into(),
            name: "paid".into(),
            provider: "mock".into(),
            context_window: 1,
            max_output_tokens: None,
            supports_vision: false,
            supports_tools: true,
            supports_streaming: false,
            input_cost_per_million: Some(1.0),
            output_cost_per_million: Some(1.0),
        }])
    }

    async fn complete(&self, _: CompletionRequest) -> Result<CompletionResponse> {
        anyhow::bail!("unused")
    }

    async fn complete_stream(
        &self,
        _: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        anyhow::bail!("unused")
    }
}

#[tokio::test]
async fn durable_spawn_fails_when_session_store_is_unwritable() {
    let dir = tempfile::tempdir().expect("tempdir");
    unsafe {
        std::env::set_var("CODETETHER_DATA_DIR", dir.path().join("blocked"));
    }
    std::fs::write(dir.path().join("blocked"), "not a directory").expect("blocker");
    let mut registry = crate::provider::ProviderRegistry::new();
    registry.register(Arc::new(MockProvider));
    super::registry::set_registry_for_test(Arc::new(registry)).await;

    let result = AgentTool::new()
        .execute(json!({
            "action": "spawn", "name": "persist_fail",
            "instructions": "test", "model": "mock/paid:free"
        }))
        .await
        .expect("tool result");

    assert!(!result.success);
    assert!(result.output.contains("child session persistence failed"));
}
