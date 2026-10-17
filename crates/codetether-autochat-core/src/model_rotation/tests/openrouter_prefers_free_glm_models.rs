//! Existing model selection regression coverage.
use super::*;

#[test]
fn openrouter_prefers_free_glm_models() {
    let models = vec![
        model("openai/gpt-4.1"),
        model("z-ai/glm-5:free"),
        model("moonshot/kimi-k2:free"),
    ];
    assert_eq!(
        choose_openrouter_model(&models).as_deref(),
        Some("z-ai/glm-5:free")
    );
}
