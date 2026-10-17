//! Vertex model identifier and URL construction.
use crate::*;
impl VertexAnthropicProvider {
    /// Build the Vertex AI URL for a specific model
    pub(crate) fn build_model_url(&self, model: &str) -> String {
        // Normalize model ID to Vertex format
        let model_id = model
            .trim_start_matches("vertex-anthropic/")
            .trim_start_matches("anthropic/")
            .trim_start_matches("claude-");

        // Handle both formats: "claude-sonnet-4-6" and "sonnet-4-6"
        let final_model_id = if model_id.starts_with("claude-") {
            model_id.to_string()
        } else {
            format!("claude-{model_id}")
        };

        format!("{}/{}:rawPredict", self.base_url, final_model_id)
    }
}
