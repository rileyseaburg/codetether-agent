use super::model_selection::resolve;

#[test]
fn preserves_luna_fast_high_model_reference() {
    let providers = vec!["openai-codex"];
    let selected = resolve(Some("openai-codex/gpt-5.6-luna-fast:high"), &providers)
        .expect("explicit provider should resolve");

    assert_eq!(
        selected.requested_model.as_deref(),
        Some("openai-codex/gpt-5.6-luna-fast:high")
    );
    assert_eq!(selected.resolved_provider, "openai-codex");
    assert_eq!(selected.resolved_model, "gpt-5.6-luna-fast:high");
}

#[test]
fn rejects_an_unavailable_explicit_provider() {
    let error = resolve(Some("openai-codex/gpt-5.6-luna-fast:high"), &["zai".into()])
        .expect_err("missing explicit provider should fail");

    assert!(error.to_string().contains("openai-codex"));
    assert!(error.to_string().contains("zai"));
}
