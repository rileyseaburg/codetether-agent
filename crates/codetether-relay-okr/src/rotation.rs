//! Model rotation policy for interactive relay commands.
use crate::{GO_SWAP_MODEL_GLM, GO_SWAP_MODEL_MINIMAX};

fn is_glm5_model(model: &str) -> bool {
    let normalized = model.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "zai/glm-5"
            | "z-ai/glm-5"
            | "openrouter/z-ai/glm-5"
            | "glm5/glm-5-fp8"
            | "glm5/glm-5"
            | "glm5:glm-5-fp8"
            | "glm5:glm-5"
    )
}

fn is_minimax_model(model: &str) -> bool {
    let normalized = model.trim().to_ascii_lowercase();
    normalized.contains("minimax")
}

/// Rotate between available models for the /go command.
/// Alternates between MiniMax and GLM-5 based on the current model.
/// # Arguments
/// * `current_model` - Current model name, if known.
/// # Returns
/// The next canonical GLM or MiniMax model name.
/// # Examples
/// ```
/// assert_eq!(codetether_relay_okr::next_go_model(None), "minimax/MiniMax-M3");
/// ```
pub fn next_go_model(current_model: Option<&str>) -> String {
    match current_model {
        Some(model) if is_glm5_model(model) => GO_SWAP_MODEL_MINIMAX.to_string(),
        Some(model) if is_minimax_model(model) => GO_SWAP_MODEL_GLM.to_string(),
        _ => GO_SWAP_MODEL_MINIMAX.to_string(),
    }
}
