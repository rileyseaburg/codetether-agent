//! Fallback model catalog.
use crate::*;
impl CopilotProvider {
    /// Known models to use as a fallback when the /models API is unreachable.
    pub(crate) fn known_models(&self) -> Vec<ModelInfo> {
        rows::all()
            .map(|(id, name, ctx, max_out, vision)| ModelInfo {
                id: id.to_string(),
                name: name.to_string(),
                provider: self.provider_name.clone(),
                context_window: *ctx,
                max_output_tokens: Some(*max_out),
                supports_vision: *vision,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            })
            .collect()
    }
}
pub(crate) mod filter;
mod rows;
