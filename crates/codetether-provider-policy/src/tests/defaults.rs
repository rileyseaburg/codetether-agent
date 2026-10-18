//! Original default-model regression cases.

use crate::default_model_for_provider;

#[test]
fn openai_defaults_to_gpt4o() {
    assert_eq!(default_model_for_provider("openai"), "gpt-4o");
}

#[test]
fn openai_codex_defaults_to_gpt_5_5() {
    assert_eq!(default_model_for_provider("openai-codex"), "gpt-5.5");
}

#[test]
fn unknown_provider_defaults_to_gpt4o() {
    assert_eq!(default_model_for_provider("does-not-exist"), "gpt-4o");
}

#[test]
fn zhipuai_aliases_resolve_to_glm() {
    assert_eq!(default_model_for_provider("zai"), "glm-5");
    assert_eq!(default_model_for_provider("zhipuai"), "glm-5");
    assert_eq!(default_model_for_provider("zai-api"), "glm-5");
}

#[test]
fn novita_default_uses_canonical_model_id() {
    assert_eq!(default_model_for_provider("novita"), "qwen/qwen3.5-35b-a3b");
}
