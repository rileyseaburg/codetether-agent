use super::resolve;

#[test]
fn preferred_provider_is_deterministic_across_input_order() {
    for providers in [vec!["zai", "openai-codex"], vec!["openai-codex", "zai"]] {
        let selected = resolve(None, &providers).expect("default model");
        assert_eq!(selected.resolved_provider, "openai-codex");
    }
}

#[test]
fn unknown_provider_uses_lexical_order() {
    for providers in [vec!["custom-z", "custom-a"], vec!["custom-a", "custom-z"]] {
        let selected = resolve(None, &providers).expect("default model");
        assert_eq!(selected.resolved_provider, "custom-a");
    }
}
