//! Construct the direct OpenRouter request body.
use super::config::morph_model_ref;
use serde_json::{Value, json};
pub(super) fn body(prompt: &str) -> Value {
    let model_ref = morph_model_ref();
    let model = if let Some(stripped) = model_ref.strip_prefix("openrouter/") {
        stripped.to_string()
    } else {
        model_ref
    };
    let body = json!({
        "model": model,
        "messages": [{
            "role": "user",
            "content": prompt
        }]
    });

    body
}
