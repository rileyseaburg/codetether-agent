//! Apply Groq's advertised output ceilings independently of context windows.
use super::OpenAIProvider;
use codetether_provider_core::{CompletionRequest, ModelInfo};

impl OpenAIProvider {
    pub(super) async fn groq_model_limits(&self) -> Option<&[ModelInfo]> {
        self.groq_models
            .get_or_try_init(|| async {
                let models = self.discover_models_from_api().await;
                // Do not permanently cache a discovery outage or empty catalog.
                if models.is_empty() {
                    Err(())
                } else {
                    Ok(models)
                }
            })
            .await
            .ok()
            .map(Vec::as_slice)
    }

    pub(super) async fn cap_groq_output(&self, request: &mut CompletionRequest) {
        let Some(requested) = request.max_tokens else {
            return;
        };
        let Some(models) = self.groq_model_limits().await else {
            return;
        };
        let Some(limit) = models
            .iter()
            .find(|model| model.id == request.model)
            .and_then(|model| model.max_output_tokens)
            .filter(|limit| *limit > 0)
        else {
            return;
        };
        if requested > limit {
            tracing::debug!(provider = %self.provider_name, model = %request.model,
                requested, limit, "Clamping completion tokens to model output limit");
            request.max_tokens = Some(limit);
        }
    }
}
