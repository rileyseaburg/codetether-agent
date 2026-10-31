//! Mocked local instance metadata drives the real history derivation path.
mod support;
use async_trait::async_trait;
use codetether_context_policy::derivation::DerivePolicy;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, ModelInfo, Provider, StreamChunk,
};
use codetether_request_tokens::estimate_request_tokens;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

struct LocalModel(AtomicUsize);
#[async_trait]
impl Provider for LocalModel {
    fn name(&self) -> &str {
        "lmstudio"
    }
    async fn list_models(&self) -> anyhow::Result<Vec<ModelInfo>> {
        Ok(vec![ModelInfo {
            id: "fixture".into(),
            name: "fixture".into(),
            provider: "lmstudio".into(),
            context_window: self.0.load(Ordering::SeqCst),
            max_output_tokens: Some(1024),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: None,
            output_cost_per_million: None,
        }])
    }
    async fn complete(&self, request: CompletionRequest) -> anyhow::Result<CompletionResponse> {
        support::NoCalls.complete(request).await
    }
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> anyhow::Result<futures::stream::BoxStream<'static, StreamChunk>> {
        support::NoCalls.complete_stream(request).await
    }
}

#[tokio::test]
async fn lmstudio_resumed_history_obeys_loaded_capacity_and_reload() {
    let session = support::Session::new();
    let original = serde_json::to_string(&session.messages).unwrap();
    let provider = Arc::new(LocalModel(AtomicUsize::new(32768)));
    for (window, budget) in [(32768, 16384), (4608, 2304)] {
        provider.0.store(window, Ordering::SeqCst);
        let derived = codetether_context_dispatch::derive_with_policy::<_, ()>(
            &session,
            provider.clone(),
            "fixture",
            "fixed policy",
            &[],
            None,
            DerivePolicy::Incremental {
                budget_tokens: 128000,
            },
            Some(1000),
            support::hooks(),
        )
        .await
        .unwrap();
        assert!(estimate_request_tokens("fixed policy", &derived.messages, &[]) <= budget);
        let encoded = serde_json::to_string(&derived.messages).unwrap();
        assert!(encoded.contains("permanent constraint"));
        assert!(encoded.contains("Investigate the requested implementation"));
        assert_eq!(serde_json::to_string(&session.messages).unwrap(), original);
    }
}

#[tokio::test]
async fn lmstudio_oversized_fixed_prompt_is_not_silently_dropped() {
    let session = support::Session::new();
    let result = codetether_context_dispatch::derive_with_policy::<_, ()>(
        &session,
        Arc::new(LocalModel(AtomicUsize::new(4608))),
        "fixture",
        &"required instruction ".repeat(10000),
        &[],
        None,
        DerivePolicy::Legacy,
        None,
        support::hooks(),
    )
    .await;
    assert!(result.is_err());
}
