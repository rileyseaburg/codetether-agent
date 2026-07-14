fn parses_model_suffix_for_thinking_level() {
    let (model, level) =
        OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.3-codex:high");
    assert_eq!(model, "gpt-5.3-codex");
    assert_eq!(level.map(ThinkingLevel::as_str), Some("high"));

    let (model, level) = OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.5:none");
    assert_eq!(model, "gpt-5.5");
    assert_eq!(level, None);

    let (model, level) = OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.5:xhigh");
    assert_eq!(model, "gpt-5.5");
    assert_eq!(level.map(ThinkingLevel::as_str), Some("xhigh"));

    // The authenticated model catalog reports `max` for all GPT-5.6 variants.
    let (model, level) = OpenAiCodexProvider::resolve_model_and_reasoning_effort("gpt-5.6-sol:max");
    assert_eq!(model, "gpt-5.6-sol");
    assert_eq!(level.map(ThinkingLevel::as_str), Some("max"));
}
