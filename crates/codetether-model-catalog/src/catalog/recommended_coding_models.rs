//! Catalog recommended coding models.
use super::*;
impl ModelCatalog {
    /// Get recommended models for coding tasks
    pub fn recommended_coding_models(&self) -> Vec<(&str, &ApiModelInfo)> {
        let preferred_ids = [
            "claude-sonnet-4-6",
            "claude-sonnet-4-20250514",
            "claude-opus-4-6",
            "claude-opus-4-20250514",
            "gpt-5-codex",
            "gpt-5.1-codex",
            "gpt-4o",
            "gemini-3.1-pro-preview",
            "gemini-2.5-pro",
            "deepseek-v3.2",
            "step-3.5-flash",
            "glm-5-fp8",
            "glm-5",
            "z-ai/glm-5",
        ];

        let mut result = Vec::new();
        for model_id in preferred_ids {
            if let Some((provider, model)) = self.find_model(model_id) {
                result.push((provider, model));
            }
        }
        result
    }
}
