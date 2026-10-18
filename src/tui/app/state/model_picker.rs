//! Application model-refresh tasks and state updates.
use codetether_model_discovery::load_available_models;
pub use codetether_model_discovery::{ModelRefreshEvent, ModelRefreshSummary};
mod catalog_registry;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::tui::models::ViewMode;

impl super::AppState {
    pub fn start_model_refresh(&mut self, registry: Arc<ProviderRegistry>) {
        let (tx, rx) = mpsc::unbounded_channel();
        self.model_refresh_rx = Some(rx);
        self.model_refresh_in_flight = true;

        tokio::spawn(async move {
            let (models, summary) = load_available_models(registry.as_ref()).await;
            let _ = tx.send(ModelRefreshEvent::Loaded { models, summary });
        });
    }

    pub fn drain_model_refresh(&mut self) {
        let mut latest = None;
        if let Some(rx) = self.model_refresh_rx.as_mut() {
            while let Ok(event) = rx.try_recv() {
                latest = Some(event);
            }
        }

        let Some(ModelRefreshEvent::Loaded { models, summary }) = latest else {
            return;
        };

        self.finish_model_refresh(models);

        if let Some(target) = self.model_picker_target_model.as_deref()
            && let Some(index) = self
                .filtered_models()
                .iter()
                .position(|model| *model == target)
        {
            self.selected_model_index = index;
        }

        if self.model_picker_active || self.view_mode == ViewMode::Model {
            self.status = summary.status_message();
        }
    }

    /// Refresh the available models list from the provider registry.
    ///
    /// # Errors
    ///
    /// Returns an error if any provider call fails critically.
    pub async fn refresh_available_models(
        &mut self,
        registry: Option<&Arc<ProviderRegistry>>,
    ) -> anyhow::Result<ModelRefreshSummary> {
        let Some(registry) = registry else {
            self.available_models.clear();
            return Ok(ModelRefreshSummary::empty());
        };

        let (models, summary) = load_available_models(registry.as_ref()).await;
        self.set_available_models(models);
        Ok(summary)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use async_trait::async_trait;
    use futures::stream::BoxStream;

    use super::*;
    use crate::provider::{
        CompletionRequest, CompletionResponse, EmbeddingRequest, EmbeddingResponse, ModelInfo,
        Provider, StreamChunk,
    };

    struct StaticProvider {
        name: &'static str,
        models: Vec<ModelInfo>,
    }

    #[async_trait]
    impl Provider for StaticProvider {
        fn name(&self) -> &str {
            self.name
        }

        async fn list_models(&self) -> Result<Vec<ModelInfo>> {
            Ok(self.models.clone())
        }

        async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
            unimplemented!("not used by model picker tests")
        }

        async fn complete_stream(
            &self,
            _request: CompletionRequest,
        ) -> Result<BoxStream<'static, StreamChunk>> {
            unimplemented!("not used by model picker tests")
        }

        async fn embed(&self, _request: EmbeddingRequest) -> Result<EmbeddingResponse> {
            unimplemented!("not used by model picker tests")
        }
    }

    fn model(id: &str, provider: &str) -> ModelInfo {
        ModelInfo {
            id: id.to_string(),
            name: id.to_string(),
            provider: provider.to_string(),
            context_window: 128_000,
            max_output_tokens: Some(16_384),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: None,
            output_cost_per_million: None,
        }
    }

    #[tokio::test]
    async fn refresh_includes_non_codex_provider_models() {
        let mut registry = ProviderRegistry::new();
        registry.register(Arc::new(StaticProvider {
            name: "openrouter",
            models: vec![model("openai/gpt-5.5", "openrouter")],
        }));
        let registry = Arc::new(registry);
        let mut state = super::super::AppState::default();

        let summary = state
            .refresh_available_models(Some(&registry))
            .await
            .expect("refresh should succeed");

        assert_eq!(state.available_models, ["openrouter/openai/gpt-5.5"]);
        assert_eq!(summary.loaded_models, 1);
        assert_eq!(summary.loaded_providers, 1);
    }

    include!("model_picker/codex_models_tests.rs");
}
