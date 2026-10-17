//! Canonical GLM model names shared with provider routing.

/// Default model served by GLM-5 deployments.
pub const DEFAULT_MODEL: &str = "glm-5-fp8";

/// Normalize a model string: strip provider prefix (`glm5/`, `glm5:`) and
/// map aliases to the canonical model name served by vLLM.
///
/// Examples:
/// - `"glm5/glm-5-fp8"` → `"glm-5-fp8"`
/// - `"glm5:glm-5-fp8"` → `"glm-5-fp8"`
/// - `"glm-5-fp8"`      → `"glm-5-fp8"`
/// - `"glm-5"`          → `"glm-5-fp8"` (alias to fp8 variant)
/// - `""`               → `DEFAULT_MODEL`
pub fn normalize_model(model: &str) -> String {
    let stripped = model
        .trim()
        .trim_start_matches("glm5/")
        .trim_start_matches("glm5:");

    match stripped {
        "" | "glm-5" | "glm5" => DEFAULT_MODEL.to_string(),
        other => other.to_string(),
    }
}
