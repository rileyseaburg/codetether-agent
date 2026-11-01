//! Loaded LM Studio capacity, never the architecture's advertised maximum.
//!
//! Discovery and each chat submission read fresh instance metadata. No global
//! model-ID overrides are installed, so another provider serving the same model
//! keeps its own capacity. Unknown/unloaded capacity fails closed.

mod budget;
mod config;
mod discovery;
mod dispatch;
mod gate;
mod metadata;
mod registry;
mod stream;

use super::*;
use anyhow::Context;

impl OpenAIProvider {
    pub(super) fn is_lmstudio(&self) -> bool {
        matches!(self.provider_name.as_str(), "lmstudio" | "lm-studio")
    }

    pub(super) async fn guard_lmstudio_request(
        &self,
        mut request: CompletionRequest,
    ) -> Result<CompletionRequest> {
        if !self.is_lmstudio() {
            return Ok(request);
        }
        let models = self.lmstudio_models().await?;
        let model = models.iter().find(|model| model.id == request.model)
            .with_context(|| format!(
                "LM Studio model '{}' has no loaded_context_length; load the model before sending a request",
                request.model,
            ))?;
        budget::cap_output(
            &mut request,
            model.context_window,
            config::Config::load()?.output,
        )?;
        Ok(request)
    }
}
