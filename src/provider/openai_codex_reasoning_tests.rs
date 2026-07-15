#[test]
fn ignores_unknown_model_suffix() {
    let (model, level) =
        OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.3-codex:turbo");
    assert_eq!(model, "gpt-5.3-codex:turbo");
    assert_eq!(level, None);

    let (model, level) = OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.5:minimal");
    assert_eq!(model, "gpt-5.5:minimal");
    assert_eq!(level, None);
}

#[test]
fn preserves_ultra_but_sends_max_on_the_wire() {
    let (model, level) =
        OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.6-sol:ultra");
    assert_eq!(model, "gpt-5.6-sol");
    assert_eq!(level.map(ThinkingLevel::as_str), Some("ultra"));
    assert_eq!(level.map(ThinkingLevel::as_wire_str), Some("max"));
}

#[test]
fn composes_fast_service_tier_with_ultra_reasoning() {
    let (model, level, tier) =
        OpenAiCodexProvider::resolve_model_and_reasoning_effort_and_service_tier(
            "gpt-5.6-sol-fast:ultra",
        );
    assert_eq!(model, "gpt-5.6-sol");
    assert_eq!(level.map(ThinkingLevel::as_str), Some("ultra"));
    assert_eq!(level.map(ThinkingLevel::as_wire_str), Some("max"));
    assert_eq!(tier.map(CodexServiceTier::as_str), Some("priority"));
}

#[test]
fn skips_reasoning_effort_for_non_reasoning_models() {
    let (model, level) = OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-4o:high");
    assert_eq!(model, "gpt-4o");
    assert_eq!(level, None);
}
