//! Shared model normalization API.
use crate::*;
impl Glm5Provider {
    /// Normalize a model string: strip provider prefix (`glm5/`, `glm5:`) and
    /// map aliases to the canonical model name served by vLLM.
    ///
    /// Examples:
    /// - `"glm5/glm-5-fp8"` → `"glm-5-fp8"`
    /// - `"glm5:glm-5-fp8"` → `"glm-5-fp8"`
    /// - `"glm-5-fp8"`      → `"glm-5-fp8"`
    /// - `"glm-5"`          → `"glm-5-fp8"` (alias to fp8 variant)
    /// - `""`               → `DEFAULT_MODEL`
    ///
    /// # Arguments
    /// * `model` - Model identifier or provider-prefixed alias.
    ///
    /// # Returns
    /// The canonical model name.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(codetether_provider_glm5::Glm5Provider::normalize_model("glm-5"), "glm-5-fp8");
    /// ```
    pub fn normalize_model(model: &str) -> String {
        codetether_provider_core::model_identity::glm5::normalize_model(model)
    }
}
