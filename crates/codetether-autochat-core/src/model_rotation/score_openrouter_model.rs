//! Auto-chat score openrouter model.
use super::*;

pub(super) fn score_openrouter_model(model_id: &str) -> i32 {
    let id = model_id.to_ascii_lowercase();
    let mut score = 0;
    if is_openrouter_free_model(&id) {
        score += 1000;
    }
    if id.contains("glm-5") {
        score += 250;
    }
    if id.contains("minimax") {
        score += 220;
    }
    if id.contains("gpt-5-mini") {
        score += 180;
    }
    if id.contains("coder") {
        score += 70;
    }
    score
}
