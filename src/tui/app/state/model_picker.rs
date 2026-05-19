//! Async model list refresh from the provider registry.

use std::sync::Arc;
use std::time::Duration;

use futures::future::join_all;
use tokio::sync::mpsc;

use crate::provider::{ModelInfo, ProviderRegistry};
use crate::tui::models::ViewMode;

const MODEL_LIST_TIMEOUT: Duration = Duration::from_secs(4);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRefreshSummary {
    pub provider_count: usize,
    pub loaded_providers: usize,
    pub loaded_models: usize,
    pub failed_providers: usize,
    pub timed_out_providers: usize,
}

impl ModelRefreshSummary {
    fn empty() -> Self {
        Self {
            provider_count: 0,
            loaded_providers: 0,
            loaded_models: 0,
            failed_providers: 0,
            timed_out_providers: 0,
        }
    }

    fn skipped_providers(&self) -> usize {
        self.failed_providers + self.timed_out_providers
    }

    fn status_message(&self) -> String {
        if self.loaded_models == 0 {
            let skipped = self.skipped_providers();
            if self.provider_count == 0 || skipped == 0 {
                "No models available".to_string()
            } else {
                format!(
                    "No models available ({} provider{} skipped)",
                    skipped,
                    plural(skipped)
                )
            }
        } else {
            let mut status = format!(
                "Model picker: {} models from {} provider{}",
                self.loaded_models,
                self.loaded_providers,
                plural(self.loaded_providers)
            );
            let skipped = self.skipped_providers();
            if skipped > 0 {
                status.push_str(&format!(" ({} skipped)", skipped));
            }
            status
        }
    }
}

#[derive(Debug)]
pub enum ModelRefreshEvent {
    Loaded {
        models: Vec<String>,
        summary: ModelRefreshSummary,
    },
}

impl super::AppState {
    pub fn start_model_refresh(&mut self, registry: Arc<ProviderRegistry>) {
        let (tx, rx) = mpsc::unbounded_channel();
        self.model_refresh_rx = Some(rx);
        self.model_refresh_in_flight = true;

        tokio::spawn(async move {
            let (models, summary) = load_available_models(&registry).await;
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

        self.set_available_models(models);
        self.model_refresh_in_flight = false;
        self.model_refresh_rx = None;

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

        let (models, summary) = load_available_models(registry).await;
        self.set_available_models(models);
        Ok(summary)
    }
}

async fn load_available_models(registry: &ProviderRegistry) -> (Vec<String>, ModelRefreshSummary) {
    let provider_names: Vec<String> = registry.list().into_iter().map(str::to_string).collect();
    let provider_count = provider_names.len();

    let fetches = provider_names.into_iter().filter_map(|provider_name| {
        registry.get(&provider_name).map(|provider| async move {
            let result = tokio::time::timeout(MODEL_LIST_TIMEOUT, provider.list_models()).await;
            (provider_name, result)
        })
    });

    let mut models = Vec::new();
    let mut summary = ModelRefreshSummary {
        provider_count,
        ..ModelRefreshSummary::empty()
    };

    for (provider_name, result) in join_all(fetches).await {
        match result {
            Ok(Ok(provider_models)) => {
                let before = models.len();
                models.extend(
                    provider_models
                        .iter()
                        .filter_map(|model| model_ref_for_provider(&provider_name, model)),
                );
                if models.len() > before {
                    summary.loaded_providers += 1;
                }
            }
            Ok(Err(err)) => {
                summary.failed_providers += 1;
                tracing::warn!(
                    provider = %provider_name,
                    error = %err,
                    "failed to load models for TUI picker"
                );
            }
            Err(_) => {
                summary.timed_out_providers += 1;
                tracing::warn!(
                    provider = %provider_name,
                    timeout_ms = MODEL_LIST_TIMEOUT.as_millis(),
                    "timed out loading models for TUI picker"
                );
            }
        }
    }

    models.sort();
    models.dedup();
    summary.loaded_models = models.len();
    (models, summary)
}

fn model_ref_for_provider(provider_name: &str, model: &ModelInfo) -> Option<String> {
    let provider_name = provider_name.trim();
    let model_id = model.id.trim();
    if provider_name.is_empty() || model_id.is_empty() {
        return None;
    }

    let provider_prefix = format!("{provider_name}/");
    if model_id.starts_with(&provider_prefix) {
        Some(model_id.to_string())
    } else {
        Some(format!("{provider_name}/{model_id}"))
    }
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use async_trait::async_trait;
    use futures::stream::BoxStream;

    use super::*;
    use crate::provider::{
        CompletionRequest, CompletionResponse, EmbeddingRequest, EmbeddingResponse, Provider,
        StreamChunk,
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
    async fn refresh_prefixes_nested_model_ids_with_registry_provider() {
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

        assert_eq!(state.available_models, vec!["openrouter/openai/gpt-5.5"]);
        assert_eq!(summary.loaded_models, 1);
        assert_eq!(summary.loaded_providers, 1);
    }

    #[tokio::test]
    async fn refresh_does_not_double_prefix_provider_qualified_ids() {
        let mut registry = ProviderRegistry::new();
        registry.register(Arc::new(StaticProvider {
            name: "openrouter",
            models: vec![model("openrouter/z-ai/glm-5", "openrouter")],
        }));
        let registry = Arc::new(registry);
        let mut state = super::super::AppState::default();

        state
            .refresh_available_models(Some(&registry))
            .await
            .expect("refresh should succeed");

        assert_eq!(state.available_models, vec!["openrouter/z-ai/glm-5"]);
    }
}
