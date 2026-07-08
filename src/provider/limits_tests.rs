//! Tests for [`crate::provider::limits::context_window_for_model`].

use super::context_window_for_model;

#[test]
fn known_models() {
    assert_eq!(context_window_for_model("zai/glm-5"), 200_000);
    assert_eq!(context_window_for_model("glm-5.2"), 1_000_000);
    assert_eq!(context_window_for_model("glm-5-pro"), 200_000);
    assert_eq!(context_window_for_model("kimi-k2.5"), 256_000);
    assert_eq!(
        context_window_for_model("anthropic.claude-opus-4-7-v1:0"),
        1_000_000
    );
    assert_eq!(context_window_for_model("claude-3-5-sonnet"), 200_000);
    assert_eq!(context_window_for_model("gpt-4o"), 128_000);
    assert_eq!(context_window_for_model("gpt-4o-mini"), 128_000);
    assert_eq!(context_window_for_model("gemini-2.0-flash"), 1_000_000);
    assert_eq!(context_window_for_model("gemini-1.5-pro"), 1_000_000);
    assert_eq!(context_window_for_model("minimax/m3"), 1_000_000);
    assert_eq!(context_window_for_model("minimax-m2.5"), 256_000);
    assert_eq!(context_window_for_model("qwen-2.5-coder"), 131_072);
    assert_eq!(context_window_for_model("deepseek-v4-flash"), 1_048_576);
    assert_eq!(context_window_for_model("deepseek-chat"), 128_000);
    assert_eq!(context_window_for_model("deepseek-reasoner"), 128_000);
    assert_eq!(context_window_for_model("deepseek-r1"), 128_000);
}

#[test]
fn bedrock_openai_gpt_5_gets_272k_context_window() {
    assert_eq!(context_window_for_model("openai.gpt-5.6-sol"), 272_000);
    assert_eq!(context_window_for_model("openai.gpt-5.6-terra"), 272_000);
    assert_eq!(context_window_for_model("openai.gpt-5.6-luna"), 272_000);
    assert_eq!(context_window_for_model("openai.gpt-5.5"), 272_000);
    assert_eq!(context_window_for_model("gpt-5.5"), 256_000);
}

#[test]
fn case_insensitive() {
    assert_eq!(
        context_window_for_model("Claude-Opus-4-7"),
        context_window_for_model("claude-opus-4-7")
    );
}

#[test]
fn unknown_falls_back() {
    assert_eq!(context_window_for_model("totally-unknown-model"), 128_000);
}
