//! Exercise raw-router identity without loading a model or enabling Candle.

#[path = "../../src/cognition/tool_router/config.rs"]
mod config;
#[path = "../../src/cognition/tool_router/identity_prompt.rs"]
mod identity_prompt;
#[path = "../../src/cognition/tool_router/router_config.rs"]
mod router_config;
use config::ToolRouterConfig;

#[test]
fn functiongemma_identity_uses_selected_checkpoint_and_runtime_snapshot() {
    let router = ToolRouterConfig::default();
    let mut config = router_config::thinker_config(&router, "functiongemma.gguf", "tokenizer.json");
    assert_eq!(config.model, "functiongemma.gguf");
    assert_eq!(
        config.candle_model_path.as_deref(),
        Some("functiongemma.gguf")
    );
    assert_eq!(
        config.candle_tokenizer_path.as_deref(),
        Some("tokenizer.json")
    );
    assert!(config.enabled);
    assert_eq!(config.max_tokens, router.max_tokens);
    assert_eq!(config.temperature, router.temperature);
    let loaded = crate::native_model_identity::NativeModelIdentity::from_config(&config);
    config.model = "next-model.gguf".to_owned();
    assert_eq!(loaded.model(), "functiongemma.gguf");
}

#[test]
fn functiongemma_identity_preserves_system_and_conversation() {
    let system = "Extract <tool_call> blocks.\nKeep Unicode: café 🎥.";
    let rest = "\n<start_of_turn>user\nRead file\n<end_of_turn>\n<start_of_turn>model\n";
    let prompt = format!("<start_of_turn>system\n{system}<end_of_turn>{rest}");
    let actual = identity_prompt::with_identity(&prompt, "functiongemma.gguf");
    let expected = crate::identity::system_prompt(system, "candle", "functiongemma.gguf");
    assert_eq!(
        actual,
        format!("<start_of_turn>system\n{expected}<end_of_turn>{rest}")
    );
    assert_eq!(actual.matches("<start_of_turn>system\n").count(), 1);
}

#[test]
fn functiongemma_identity_handles_unformatted_prompt_without_loss() {
    for prompt in ["raw prompt 🎥", "<start_of_turn>system\nunterminated", ""] {
        let actual = identity_prompt::with_identity(prompt, "checkpoint.gguf");
        let expected = crate::identity::system_prompt("", "candle", "checkpoint.gguf");
        assert_eq!(
            actual,
            format!("<start_of_turn>system\n{expected}<end_of_turn>\n{prompt}")
        );
    }
}

#[path = "functiongemma_refresh.rs"]
mod refresh;
